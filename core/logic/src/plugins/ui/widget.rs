use crate::{
    prelude::*,
    systems::{
        ui::widget::*,
        utility::*,
    },
};

/// Plugin for managing the widgets of the user interface.
pub struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<UiFont>();
        app.register_type::<WidgetStructure>();
        app.register_type::<PreviouslySelectedWidgetStack>();
        app.register_type::<RestorePreviouslySelectedWidget>();
        app.register_type::<PreviouslySelectedWidgetRestored>();

        // Register components.
        app.register_type::<Widget>();
        app.register_type::<ButtonColors>();
        app.register_type::<ScaledFontSize>();

        // Insert resources.
        app.init_resource::<PreviouslySelectedWidgetStack>();

        // Add systems.
        app.add_systems(
            Startup,
            load_ui_font.run_if(|ui_font: Option<Res<UiFont>>| ui_font.is_none()),
        );
        app.add_systems(OnEnter(InMenu), spawn_menu_background);
        app.add_systems(OnExit(InMenu), despawn_menu_background);
        app.add_systems(
            PreUpdate,
            initialize_widget_structure.run_if(in_state(InMenu)).run_if(
                |widget_grid: Option<Res<WidgetStructure>>,
                 widget_query: Query<&Widget, Changed<Transform>>| {
                    widget_grid.is_none() || !widget_query.is_empty()
                },
            ),
        );
        app.add_systems(
            Update,
            navigation_with_menu_actions
                .run_if(in_state(InMenu))
                .run_if(console_is_not_open)
                .run_if(|menu_action_state: Res<ActionState<MenuAction>>| {
                    menu_action_state.is_changed()
                }),
        );
        app.add_systems(
            PostUpdate,
            (
                add_widget_selected_on_mouse_wiggle,
                set_is_selected_when_widget_selected_component_is_added,
                ensure_single_widget_is_selected,
                remove_widget_selected_from_widgets_which_are_not_selected,
                add_widget_clicked_on_click,
                update_widget_state_on_user_interactions,
                update_button_colors_on_state_change,
            )
                .chain()
                .run_if(in_state(InMenu))
                .run_if(console_is_not_open),
        );
        app.add_systems(
            PostUpdate,
            (
                change_button_label_color_when_disabled,
                change_button_label_color_when_enabled,
                scale_texts
                    .run_if(|parent_query: Query<&Node, Changed<Node>>| !parent_query.is_empty()),
            ),
        );
    }
}
