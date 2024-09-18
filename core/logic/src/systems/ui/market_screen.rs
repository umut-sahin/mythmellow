//! Market screen systems.

use {
    crate::{
        prelude::*,
        systems::utility::*,
    },
    constants::ui::widget::*,
    localizations::ui as localization,
};


/// Spawns the market screen.
pub fn spawn_market_screen(
    mut commands: Commands,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
    balance: Res<Balance>,
    market_configuration: Res<MarketConfiguration>,
    market_state: Res<MarketState>,
) {
    log::info!("spawning the market screen");
    commands
        .ui_builder(UiRoot)
        .column(|root| {
            // Spawn the header.
            {
                root.row(|header| {
                    // Spawn the balance container.
                    {
                        header
                            .container(NodeBundle::default(), |balance_container| {
                                balance_container.spawn((
                                    Name::new("Text"),
                                    MarketScreenBalanceText,
                                    TextBundle {
                                        text: Text::from_section(
                                            format!("{:.0}", *balance),
                                            TextStyle {
                                                color: MARKET_BALANCE_TEXT_COLOR,
                                                font: ui_font.clone(),
                                                ..default()
                                            },
                                        )
                                        .with_justify(JustifyText::Center),
                                        ..default()
                                    },
                                    ScaledFontSize { base: MARKET_BALANCE_TEXT_FONT_SIZE },
                                ));
                            })
                            .named("Balance")
                            .style()
                            .align_self(AlignSelf::Start)
                            .align_items(AlignItems::Center)
                            .width(Val::Percent(20.00))
                            .height(Val::Percent(90.00));
                    }

                    // Spawn the refresh button.
                    {
                        let refresh_cost = market_configuration.refresh_cost(&market_state);

                        let mut refresh_button = header.market_refresh_button(
                            localization::refresh_button(refresh_cost),
                            localization.deref(),
                            ui_font.clone(),
                        );
                        refresh_button
                            .named("Refresh Button")
                            .insert(MarketScreenRefreshButton { cost: refresh_cost })
                            .entity_commands()
                            .observe(on_refresh_button_clicked);

                        let raw_refresh_cost = if market_configuration.free_refreshes > 0 {
                            Balance::ZERO
                        } else {
                            market_configuration.refresh_cost.get()
                        };

                        let market_is_initialized = market_state.offered_item_ids.len()
                            == (market_configuration.number_of_items as usize);

                        if (!market_is_initialized && *balance < raw_refresh_cost)
                            || (market_is_initialized && *balance < refresh_cost)
                        {
                            refresh_button.insert(WidgetDisabled);
                        }
                    }
                })
                .named("Header")
                .style()
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::SpaceBetween)
                .width(Val::Percent(85.00))
                .height(Val::Percent(10.00));
            }

            // Spawn the offerings container.
            {
                root.row(|_| {})
                    .named("Offerings")
                    .insert(MarketScreenOfferingsContainer)
                    .style()
                    .width(Val::Percent(85.00))
                    .height(Val::Percent(50.00))
                    .column_gap(Val::Percent(3.00));
            }

            // Spawn the continue button.
            {
                root.market_continue_button(
                    localization::continue_button(),
                    localization.deref(),
                    ui_font.clone(),
                )
                .named("Continue Button")
                .insert(WidgetSelected::now())
                .entity_commands()
                .observe(on_continue_button_clicked);
            }
        })
        .named("Market")
        .insert(StateScoped(GameState::Market))
        .style()
        .width(Val::Percent(100.00))
        .height(Val::Percent(100.00))
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .row_gap(Val::Percent(4.50));
}


/// Refreshes the market.
pub fn on_refresh_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    refresh_button_query: Query<&MarketScreenRefreshButton>,
    mut balance: ResMut<Balance>,
    mut market_configuration: ResMut<MarketConfiguration>,
    market_state: ResMut<MarketState>,
    registered_systems: Res<RegisteredSystems>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    let button = match refresh_button_query.get(trigger.entity()) {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    log::info!("refresh button is clicked");

    let refresh_cost = button.cost;
    if *balance < refresh_cost {
        log::error!(
            "unable to refresh the market, which required {}, as the player only have {}",
            refresh_cost,
            *balance,
        );
        return;
    }

    let refresh_was_free_as_no_item_was_available =
        market_configuration.refresh_is_free_as_no_item_is_available(&market_state);
    let free_refresh_used =
        (market_configuration.free_refreshes > 0) && !refresh_was_free_as_no_item_was_available;

    if *refresh_cost != 0.00 {
        balance.spend(refresh_cost, "refresh the market");
    } else if free_refresh_used {
        if market_configuration.free_refreshes == 1 {
            log::info!("using the last free refresh");
        } else {
            log::info!(
                "using 1 of {} available free refreshes",
                market_configuration.free_refreshes,
            );
        }
    } else {
        log::info!("refreshing for free as no item is available to purchase in the market");
    }
    commands.run_system(registered_systems.market.refresh_market);

    if free_refresh_used {
        market_configuration.free_refreshes -= 1;
    }

    if !(refresh_was_free_as_no_item_was_available || free_refresh_used) {
        market_configuration.refresh_cost.step();
        log::info!("new refresh cost is {}", market_configuration.refresh_cost.get());
    }
}

/// Buys an item.
pub fn on_buy_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    buy_button_query: Query<&MarketScreenBuyButton>,
    mut market_state: ResMut<MarketState>,
    mut balance: ResMut<Balance>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    let button = match buy_button_query.get(trigger.entity()) {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let item_position = NonZeroUsize::new(button.item_index + 1).unwrap();
    log::info!("buy button of item {} is clicked", item_position);

    let item_cost = button.price;
    if *balance < item_cost {
        log::error!(
            "unable to buy item {} in the market, which required {} experience, \
            but only {} experience was available",
            item_position,
            item_cost,
            *balance,
        );
        return;
    }

    balance.spend(item_cost, format!("buy item {} in the market", item_position));
    market_state.acquire(item_position);
}

/// (Un)locks an item.
pub fn on_lock_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    lock_button_query: Query<&MarketScreenLockButton>,
    mut market_state: ResMut<MarketState>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    let button = match lock_button_query.get(trigger.entity()) {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let item_position = NonZeroUsize::new(button.item_index + 1).unwrap();
    log::info!("lock button of item {} is clicked", item_position);

    if market_state.is_locked(item_position) {
        market_state.unlock(item_position);
    } else {
        market_state.lock(item_position);
    }
}

/// Closes the market.
pub fn on_continue_button_clicked(
    trigger: Trigger<OnAdd, WidgetClicked>,
    mut commands: Commands,
    mut game_state_stack: ResMut<GameStateStack>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    commands.entity(trigger.entity()).remove::<WidgetClicked>();

    log::info!("continue button is clicked");
    log::info!("closing the market");

    game_state_stack.pop();
    next_game_state.set(GameState::Transition);
}


/// Updates the balance text.
pub fn update_balance_text(
    mut balance_text_query: Query<&mut Text, With<MarketScreenBalanceText>>,
    balance: Res<Balance>,
) {
    let mut balance_text = match balance_text_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    balance_text.sections[0].value = format!("{}", *balance);
}

/// Updates the refresh button.
pub fn update_refresh_button(
    mut commands: Commands,
    mut refresh_button_query: Query<(Entity, &mut MarketScreenRefreshButton)>,
    mut refresh_button_label_query: Query<&mut LocalizedText, With<MarketScreenRefreshButtonLabel>>,
    balance: Res<Balance>,
    market_configuration: Res<MarketConfiguration>,
    market_state: Res<MarketState>,
) {
    let (refresh_button_entity, mut refresh_button) = match refresh_button_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let refresh_cost = market_configuration.refresh_cost(&market_state);
    refresh_button.cost = refresh_cost;

    let mut refresh_button_text = match refresh_button_label_query.get_single_mut() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    *refresh_button_text = localization::refresh_button(refresh_cost);

    if *balance < refresh_cost {
        commands.entity(refresh_button_entity).insert(WidgetDisabled);
    } else {
        commands.entity(refresh_button_entity).remove::<WidgetDisabled>();
    }
}

/// Updates the offerings container.
pub fn update_offerings(
    mut commands: Commands,
    market_offerings_container_query: Query<Entity, With<MarketScreenOfferingsContainer>>,
    balance: Res<Balance>,
    market_state: Res<MarketState>,
    item_registry: Res<ItemRegistry>,
    localization: Res<Localization>,
    ui_font: Res<UiFont>,
) {
    let market_offerings_container_entity = match market_offerings_container_query.get_single() {
        Ok(query_result) => query_result,
        Err(_) => return,
    };

    let mut market_offerings_container = commands.entity(market_offerings_container_entity);
    market_offerings_container.despawn_descendants();

    for (item_index, item_id) in market_state.offered_item_ids.iter().enumerate() {
        let item_position = NonZeroUsize::new(item_index + 1).unwrap();
        commands
            .ui_builder(market_offerings_container_entity)
            .column(|offering_container| {
                if market_state.is_acquired(item_position) {
                    return;
                }

                let item = match item_registry.find_item_by_id(item_id) {
                    Some(item) => item,
                    None => {
                        return;
                    },
                };
                let price = item.base_price;

                offering_container
                    .column(|item_details| {
                        item_details.market_item_name(item.name(), &localization, ui_font.clone());

                        let mut buy_button = item_details.market_buy_button(
                            format!("{}", price).into(),
                            &localization,
                            ui_font.clone(),
                        );
                        buy_button
                            .named("Buy Button")
                            .insert(MarketScreenBuyButton { item_index, price });
                        if *balance < price {
                            buy_button.insert(WidgetDisabled);
                        }
                        buy_button.entity_commands().observe(on_buy_button_clicked);
                    })
                    .named("Details")
                    .style()
                    .row_gap(MARKET_ITEM_DETAILS_ROW_GAP)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::SpaceBetween)
                    .padding(UiRect::all(Val::Percent(15.00)))
                    .width(MARKET_ITEM_DETAILS_WIDTH)
                    .height(MARKET_ITEM_DETAILS_HEIGHT)
                    .background_color(Color::srgba(0.00, 0.00, 0.00, 0.75));

                offering_container
                    .market_lock_button(
                        if market_state.is_locked(item_position) {
                            localization::unlock_button()
                        } else {
                            localization::lock_button()
                        },
                        localization.deref(),
                        ui_font.clone(),
                    )
                    .named("Lock Button")
                    .insert(MarketScreenLockButton { item_index })
                    .entity_commands()
                    .observe(on_lock_button_clicked);
            })
            .named(format!("Offering {} [{}]", item_position, item_id))
            .style()
            .width(MARKET_OFFERING_CONTAINER_WIDTH)
            .height(MARKET_OFFERING_CONTAINER_HEIGHT)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::SpaceEvenly)
            .row_gap(MARKET_OFFERING_CONTAINER_ROW_GAP);
    }
}
