//! Widget systems.

use {
    crate::{
        prelude::*,
        systems::utility::*,
    },
    mythmallow_core_data::constants::ui::widget::{
        DISABLED_BUTTON_LABEL_COLOR,
        NORMAL_BUTTON_LABEL_COLOR,
    },
};

/// Loads the user interface font.
pub fn load_ui_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::info!("loading the ui font");
    commands.insert_resource(UiFont(asset_server.load("fonts/FiraSans-Bold.ttf")));
}


/// Spawns the menu background.
pub fn spawn_menu_background(
    mut commands: Commands,
    menu_background_query: Query<Entity, With<MenuBackground>>,
) {
    if !menu_background_query.is_empty() {
        return;
    }
    commands
        .ui_builder(UiRoot)
        .column(|_| {})
        .named("Menu Background")
        .insert(MenuBackground)
        .style()
        .position_type(PositionType::Absolute)
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .background_color(Color::BLACK.with_alpha(0.97))
        .z_index(ZIndex::Global(-100));
}

/// Despawns the menu background.
pub fn despawn_menu_background(
    mut commands: Commands,
    menu_background_query: Query<Entity, With<MenuBackground>>,
    game_state: Option<Res<State<GameState>>>,
    game_state_stack: Option<Res<GameStateStack>>,
) {
    if let Some(game_state) = game_state {
        if game_state.get() == &GameState::Transition {
            let next_game_state =
                game_state_stack.and_then(|game_state_stack| game_state_stack.last().cloned());
            if InMenu::compute((AppState::Game, next_game_state)).is_some() {
                // No need to despawn the menu background when transitioning to another menu.
                return;
            }
        }
    }
    if let Ok(menu_background_entity) = menu_background_query.get_single() {
        commands.entity(menu_background_entity).despawn_recursive();
    }
}


/// Initializes widget structure resource.
pub fn initialize_widget_structure(
    mut commands: Commands,
    widget_query: Query<(Entity, &GlobalTransform), With<Widget>>,
    registered_systems: Res<RegisteredSystems>,
    restore_previously_selected_widget: Option<Res<RestorePreviouslySelectedWidget>>,
) {
    log::info!("initializing the widget structure");

    let mut entities_and_global_transforms = widget_query.iter().collect::<Vec<_>>();
    entities_and_global_transforms.sort_by(|(_, global_transform1), (_, global_transform2)| {
        let x1 = global_transform1.translation().x;
        let y1 = global_transform1.translation().y;

        let x2 = global_transform2.translation().x;
        let y2 = global_transform2.translation().y;

        match y1.partial_cmp(&y2) {
            Some(ordering) if matches!(ordering, Ordering::Greater | Ordering::Less) => ordering,
            _ => x1.partial_cmp(&x2).unwrap_or(Ordering::Equal),
        }
    });

    let mut widget_structure = Vec::with_capacity(entities_and_global_transforms.len());
    for (entity, global_transform) in entities_and_global_transforms.iter() {
        enum Direction {
            Up,
            Left,
            Down,
            Right,
        }

        fn pick_closest<'i, 't: 'i>(
            entity: &Entity,
            global_transform: &GlobalTransform,
            mut candidates: impl Iterator<Item = &'i (Entity, &'t GlobalTransform)>,
            direction: Direction,
        ) -> Entity {
            match candidates.next() {
                Some((candidate, candidate_global_transform)) => {
                    let calculate_distance = |candidate_global_transform: &GlobalTransform| {
                        let x1 = global_transform.translation().x;
                        let y1 = global_transform.translation().y;

                        let x2 = candidate_global_transform.translation().x;
                        let y2 = candidate_global_transform.translation().y;

                        let mut dx = x2 - x1;
                        let mut dy = y2 - y1;

                        match direction {
                            Direction::Up | Direction::Down => {
                                dx *= 5.00;
                            },
                            Direction::Left | Direction::Right => {
                                dy *= 5.00;
                            },
                        }

                        (dx.powi(2) + dy.powi(2)).sqrt()
                    };

                    let mut up = *candidate;
                    let mut best_distance = calculate_distance(candidate_global_transform);

                    for (candidate, candidate_global_transform) in candidates {
                        let candidate_distance = calculate_distance(candidate_global_transform);
                        if candidate_distance < best_distance {
                            best_distance = candidate_distance;
                            up = *candidate;
                        }
                    }
                    up
                },
                None => *entity,
            }
        }

        let up = {
            let candidates =
                entities_and_global_transforms.iter().filter(|(_, candidate_global_transform)| {
                    candidate_global_transform.translation().y < global_transform.translation().y
                });
            pick_closest(entity, global_transform, candidates, Direction::Up)
        };
        let left = {
            let candidates =
                entities_and_global_transforms.iter().filter(|(_, candidate_global_transform)| {
                    candidate_global_transform.translation().x < global_transform.translation().x
                });
            pick_closest(entity, global_transform, candidates, Direction::Left)
        };
        let down = {
            let candidates =
                entities_and_global_transforms.iter().filter(|(_, candidate_global_transform)| {
                    candidate_global_transform.translation().y > global_transform.translation().y
                });
            pick_closest(entity, global_transform, candidates, Direction::Down)
        };
        let right = {
            let candidates =
                entities_and_global_transforms.iter().filter(|(_, candidate_global_transform)| {
                    candidate_global_transform.translation().x > global_transform.translation().x
                });
            pick_closest(entity, global_transform, candidates, Direction::Right)
        };

        let neighbors = NeighboringWidgets { up, left, down, right };
        widget_structure.push((*entity, neighbors));
    }

    commands.insert_resource(WidgetStructure(widget_structure));

    if restore_previously_selected_widget.is_some() {
        commands.remove_resource::<RestorePreviouslySelectedWidget>();
        commands.run_system(registered_systems.widget.restore_selected_widget);
        commands.init_resource::<PreviouslySelectedWidgetRestored>();
    }
}

/// Navigates between widgets with menu actions
pub fn navigation_with_menu_actions(
    mut commands: Commands,
    mut widget_query: Query<(Entity, &mut Widget), With<WidgetSelected>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    widget_structure: Option<Res<WidgetStructure>>,
    current_app_state: Res<State<AppState>>,
    current_game_state: Option<Res<State<GameState>>>,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    registered_systems: Res<RegisteredSystems>,
) {
    if menu_action_state.just_pressed(&MenuAction::Back) {
        if let Some(game_state) = current_game_state {
            match game_state.get() {
                GameState::Paused | GameState::SettingsMenu => {
                    log::info!("back action is triggered");

                    commands.insert_resource(RestorePreviouslySelectedWidget);
                    game_state_stack.pop();
                    next_game_state.set(GameState::Transition);
                },
                GameState::LevelUpScreen | GameState::Market => {
                    log::info!("pause action is triggered");

                    commands.run_system(registered_systems.widget.save_selected_widget);
                    game_state_stack.push(GameState::Paused);
                    next_game_state.set(GameState::Transition);
                },
                _ => {},
            }
        } else if let Some(previous_app_state) = current_app_state.previous() {
            log::info!("back action is triggered");
            log::info!("transitioning to {}", previous_app_state);

            commands.insert_resource(RestorePreviouslySelectedWidget);
            next_app_state.set(previous_app_state);
        }
        return;
    }

    let (entity, mut widget) = match widget_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if menu_action_state.just_pressed(&MenuAction::Click) {
        widget.is_clicked = true;
        return;
    }

    let go_up = menu_action_state.just_pressed(&MenuAction::Up);
    let go_left = menu_action_state.just_pressed(&MenuAction::Left);
    let go_down = menu_action_state.just_pressed(&MenuAction::Down);
    let go_right = menu_action_state.just_pressed(&MenuAction::Right);

    if !(go_up || go_down || go_left || go_right) {
        return;
    }

    if let Some(widget_structure) = widget_structure {
        if let Some(neighbors) = widget_structure.neighbors_of(entity) {
            if go_up && neighbors.up != entity {
                commands.entity(neighbors.up).insert(WidgetSelected::now());
            }
            if go_left && neighbors.left != entity {
                commands.entity(neighbors.left).insert(WidgetSelected::now());
            }
            if go_down && neighbors.down != entity {
                commands.entity(neighbors.down).insert(WidgetSelected::now());
            }
            if go_right && neighbors.right != entity {
                commands.entity(neighbors.right).insert(WidgetSelected::now());
            }
        }
    }
}


/// Saves the index of the selected widget to previously selected widget stack.
pub fn save_selected_widget(
    widget_query: Query<Entity, With<WidgetSelected>>,
    widget_structure: Option<Res<WidgetStructure>>,
    mut previously_selected_widget_stack: ResMut<PreviouslySelectedWidgetStack>,
) {
    let widget_structure = match widget_structure {
        Some(resource) => resource,
        None => return,
    };

    let entity = match widget_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    if let Some(index) = widget_structure.index_of(entity) {
        log::info!("saving the selected widget");
        previously_selected_widget_stack.push(index);
    }
}

/// Saves the index of the selected widget to previously selected widget stack.
pub fn restore_selected_widget(
    mut commands: Commands,
    widget_structure: Res<WidgetStructure>,
    mut previously_selected_widget_stack: ResMut<PreviouslySelectedWidgetStack>,
) {
    let previously_selected_widget_index = match previously_selected_widget_stack.pop() {
        Some(index) => index,
        None => return,
    };

    if let Some((entity, _)) = widget_structure.get(previously_selected_widget_index) {
        log::info!("restoring the selected widget");
        commands.entity(*entity).insert(WidgetSelected::now());
    }
}


/// Adds widget selected component when the mouse wiggles over a widget.
pub fn add_widget_selected_on_mouse_wiggle(
    mut commands: Commands,
    widget_query: Query<(Entity, &Interaction), With<Widget>>,
    mouse_motion_reader: EventReader<MouseMotion>,
) {
    if !mouse_motion_reader.is_empty() {
        let mut hovered = false;
        for (entity, interaction) in &widget_query {
            if *interaction == Interaction::Hovered {
                commands.entity(entity).insert(WidgetSelected::now());
                hovered = true;
                break;
            }
        }
        if !hovered {
            for (entity, interaction) in &widget_query {
                if *interaction == Interaction::Pressed {
                    commands.entity(entity).insert(WidgetSelected::now());
                    break;
                }
            }
        }
    }
}

/// Sets is selected property of widgets when widget selected component is added to them.
pub fn set_is_selected_when_widget_selected_component_is_added(
    mut widget_query: Query<&mut Widget, Added<WidgetSelected>>,
) {
    for mut widget in &mut widget_query {
        widget.is_selected = true;
    }
}

/// Ensures that there is a single widget with is selected property.
pub fn ensure_single_widget_is_selected(
    mut selected_widgets_query: Query<(&mut Widget, &WidgetSelected)>,
) {
    let mut selected_widgets = selected_widgets_query.iter_mut().collect::<Vec<_>>();
    selected_widgets.sort_by(|(_, lhs_selected), (_, rhs_selected)| {
        lhs_selected.at.cmp(&rhs_selected.at).reverse()
    });
    for (ref mut widget, _) in selected_widgets.iter_mut().skip(1) {
        widget.is_selected = false;
    }
}

/// Removes widget selected component from widgets without is selected property.
pub fn remove_widget_selected_from_widgets_which_are_not_selected(
    mut commands: Commands,
    widget_query: Query<(Entity, &Widget), (Changed<Widget>, With<WidgetSelected>)>,
) {
    for (entity, widget) in &widget_query {
        if !widget.is_selected {
            commands.entity(entity).remove::<WidgetSelected>();
        }
    }
}


/// Adds widget clicked component when a widget is clicked.
pub fn add_widget_clicked_on_click(
    mut commands: Commands,
    mut widget_query: Query<(Entity, &mut Widget), Changed<Widget>>,
) {
    for (entity, mut widget) in widget_query.iter_mut() {
        widget.on_click(|| {
            commands.entity(entity).insert(WidgetClicked);
        });
        break;
    }
}


/// Updates the widget state on user interactions.
pub fn update_widget_state_on_user_interactions(
    mut commands: Commands,
    mut widget_query: Query<(Entity, &mut Widget, &Interaction), Changed<Interaction>>,
    previously_selected_widget_restored: Option<Res<PreviouslySelectedWidgetRestored>>,
) {
    if previously_selected_widget_restored.is_some() {
        commands.remove_resource::<PreviouslySelectedWidgetRestored>();
        return;
    }

    for (entity, mut widget, interaction) in &mut widget_query {
        match interaction {
            Interaction::None => {
                widget.is_hovered = false;
                widget.is_pressed = false;
            },
            Interaction::Hovered => {
                if widget.is_pressed {
                    widget.is_clicked = true;
                }

                commands.entity(entity).insert(WidgetSelected::now());

                widget.is_selected = true;
                widget.is_hovered = true;
                widget.is_pressed = false;
            },
            Interaction::Pressed => {
                widget.is_selected = true;
                widget.is_hovered = true;
                widget.is_pressed = true;
            },
        }
    }
}

/// Updates the colors of the buttons when their state change.
pub fn update_button_colors_on_state_change(
    mut widget_query: Query<(&Widget, &ButtonColors, &mut BackgroundColor), Changed<Widget>>,
) {
    for (widget, button_colors, mut background_color) in &mut widget_query {
        let new_background_color = if widget.is_pressed {
            button_colors.pressed_background_color.into()
        } else if widget.is_selected {
            button_colors.selected_background_color.into()
        } else {
            button_colors.normal_background_color.into()
        };
        *background_color = new_background_color;
    }
}


/// Updates the font size of the texts based on the size of their parent.
pub fn scale_texts(
    mut text_query: Query<(&Parent, &ScaledFontSize, &mut Text), With<Text>>,
    parent_query: Query<&Node, Changed<Node>>,
) {
    for (parent, font_size, mut text) in &mut text_query {
        match parent_query.get(parent.get()) {
            Ok(parent) => {
                let parent_size = parent.size();

                let width_ratio = parent_size.x / 600.00;
                let height_ratio = parent_size.y / 150.00;

                let scale = width_ratio.min(height_ratio);
                for section in text.sections.iter_mut() {
                    section.style.font_size = font_size.base * scale
                }
            },
            Err(_) => continue,
        }
    }
}


/// Changes the text color of the button labels to disabled color when widget is disabled.
pub fn change_button_label_color_when_disabled(
    mut text_query: Query<(&Parent, &mut Text)>,
    widget_query: Query<Entity, (With<Widget>, Added<WidgetDisabled>, With<Button>)>,
) {
    for (parent, mut text) in text_query.iter_mut() {
        if widget_query.get(parent.get()).is_ok() {
            text.sections[0].style.color = DISABLED_BUTTON_LABEL_COLOR;
        }
    }
}

/// Changes the text color of the button labels to normal color when widget is enabled.
pub fn change_button_label_color_when_enabled(
    mut removed_components: RemovedComponents<WidgetDisabled>,
    mut text_query: Query<(&Parent, &mut Text)>,
    widget_query: Query<Entity, (With<Widget>, With<Button>)>,
) {
    for widget_entity in removed_components.read() {
        if !widget_query.contains(widget_entity) {
            continue;
        }
        for (parent, mut text) in text_query.iter_mut() {
            if parent.get() == widget_entity {
                text.sections[0].style.color = NORMAL_BUTTON_LABEL_COLOR;
                break;
            }
        }
    }
}
