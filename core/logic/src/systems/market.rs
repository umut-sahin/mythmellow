//! Market systems.

use crate::prelude::*;


/// Resets the market.
pub fn initialize_market(mut commands: Commands) {
    log::info!("initializing the market the market");
    commands.insert_resource(Balance::default());
    commands.insert_resource(ExperienceToBalanceRatio::default());
    commands.insert_resource(MarketConfiguration::default());
    commands.insert_resource(MarketState::default());
}

/// Configures the market for the game mode.
pub fn configure_market(
    mut market_configuration: ResMut<MarketConfiguration>,
    game_mode_registry: Res<GameModeRegistry>,
    game_mode_index: Res<GameModeIndex>,
) {
    let game_mode = &game_mode_registry[game_mode_index.0];
    market_configuration.can_be_opened_by_player = game_mode.market_can_be_opened_by_player();
}


/// Gains balance when player earns experience.
pub fn gain_balance(
    mut event_reader: EventReader<ExperienceGainedEvent>,
    player_query: Query<&Player>,
    mut balance: ResMut<Balance>,
    experience_to_balance_ratio: Res<ExperienceToBalanceRatio>,
) {
    for event in event_reader.read() {
        if player_query.contains(event.entity) {
            let amount = Balance(event.experience.0 * experience_to_balance_ratio.0);
            balance.gain(
                amount,
                format!(
                    "gaining {} experience by {} (1.00 experience = {})",
                    event.experience,
                    event.by,
                    Balance(experience_to_balance_ratio.0),
                ),
            );
        }
    }
}


/// Processes acquirements from the market.
pub fn process_acquirements(
    mut market_state: ResMut<MarketState>,
    mut inventory: ResMut<Inventory>,
    item_registry: Res<ItemRegistry>,
) {
    while market_state.processed_acquirements < market_state.acquired_item_indices.len() {
        let index_of_item_to_acquire =
            market_state.acquired_item_indices[market_state.processed_acquirements];

        let item_id_to_acquire = &market_state.offered_item_ids[index_of_item_to_acquire];
        if let Some(item_to_acquire) = item_registry.find_item_by_id(item_id_to_acquire) {
            inventory.add(item_to_acquire.instantiate());
        }

        market_state.processed_acquirements += 1;
    }
}


/// Refreshes the items offered in the market.
pub fn refresh_market_automatically(world: &mut World) {
    let market_configuration = world.resource::<MarketConfiguration>();
    let number_of_items = market_configuration.number_of_items as usize;

    let mut market_state = world.resource_mut::<MarketState>();
    let mut previous_locked_item_count = 0;

    for item_index in 0..market_state.offered_item_ids.len() {
        let item_position = NonZeroUsize::new(item_index + 1).unwrap();
        if !market_state.is_acquired(item_position) {
            if market_state.is_locked(item_position) {
                previous_locked_item_count += 1;
            } else {
                market_state.locked_item_indices.push(item_index);
            }
        }
    }
    market_state.locked_item_indices.truncate(number_of_items);

    world.run_system_once(refresh_market);

    let mut market_state = world.resource_mut::<MarketState>();
    market_state.locked_item_indices.truncate(previous_locked_item_count);
}

/// Refreshes the items offered in the market.
pub fn refresh_market(
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    item_registry: Res<ItemRegistry>,
    market_configuration: Res<MarketConfiguration>,
    mut market_state: ResMut<MarketState>,
) {
    log::info!("refreshing the market to offer {} items", market_configuration.number_of_items);

    let mut new_offered_item_ids =
        Vec::with_capacity(market_configuration.number_of_items as usize);

    let mut seen_locked_item_indices = HashSet::new();
    for locked_item_index in market_state.locked_item_indices.iter().cloned() {
        let locked_item_position = NonZeroUsize::new(locked_item_index + 1).unwrap();
        if market_state.is_acquired(locked_item_position) {
            continue;
        }

        if let Some(previously_offered_item_id) =
            market_state.offered_item_ids.get(locked_item_index)
        {
            if seen_locked_item_indices.contains(&locked_item_index) {
                continue;
            }
            seen_locked_item_indices.insert(locked_item_index);

            if new_offered_item_ids.len() < (market_configuration.number_of_items as usize) {
                let price = item_registry
                    .find_item_by_id(previously_offered_item_id)
                    .map(|item| item.base_price)
                    .unwrap_or(Balance(f64::NAN));
                log::info!(
                    "re-offering locked \"{}\" at position {} in the market for {}",
                    previously_offered_item_id,
                    locked_item_position,
                    price,
                );
                new_offered_item_ids.push(previously_offered_item_id.clone());
            } else {
                log::error!(
                    "unable to re-offer locked \"{}\" at position {} in the market \
                    as the market already offers {} items",
                    previously_offered_item_id,
                    locked_item_position,
                    market_configuration.number_of_items,
                );
            }
        } else {
            log::warn!(
                "unable to re-offer the locked item at position {} in the market \
                    as it doesn't exist",
                locked_item_position,
            );
        }
    }

    let new_locked_item_indices = (0..new_offered_item_ids.len()).collect();

    if new_offered_item_ids.len() < (market_configuration.number_of_items as usize) {
        let mut commonness_of_items_that_can_be_offered = Vec::new();
        for entry in item_registry.iter() {
            let commonness = commonness_of(&market_configuration, &entry.item);
            if commonness != 0 {
                commonness_of_items_that_can_be_offered.push((
                    entry.item.id(),
                    entry.item.base_price,
                    commonness,
                ));
            }
        }
        commonness_of_items_that_can_be_offered.sort_by(
            |(id1, _, commonness1), (id2, _, commonness)| {
                if commonness1 == commonness {
                    id1.cmp(id2)
                } else {
                    commonness1.cmp(commonness).reverse()
                }
            },
        );

        let number_of_items_to_offer_randomly =
            (market_configuration.number_of_items as usize) - new_offered_item_ids.len();
        if commonness_of_items_that_can_be_offered.is_empty() {
            log::error!(
                "unable to randomly select {} more item{} to offer in the market \
                as no item is eligible to be offered in the market",
                number_of_items_to_offer_randomly,
                if number_of_items_to_offer_randomly == 1 { "" } else { "s" },
            );
        } else {
            let total_commonness = commonness_of_items_that_can_be_offered
                .iter()
                .map(|(_, _, commonness)| commonness)
                .sum::<u64>();

            let mut probability_table = Table::new();
            probability_table.add_row(row![c -> "Item", c -> "Chance", c -> "Probability"]);
            for (id, _, commonness) in commonness_of_items_that_can_be_offered.iter() {
                probability_table.add_row(row![
                    l -> id,
                    r -> format!("({} / {})", commonness, total_commonness),
                    r -> format!(
                        "{:.6}%",
                        ((*commonness as f64) / (total_commonness as f64)) * 100.00,
                    )
                ]);
            }
            let probability_table = probability_table.to_string();

            log::info!(
                "{}item{} to offer will be selected randomly with these probabilities:\n{}",
                if new_offered_item_ids.is_empty() {
                    "".to_owned()
                } else {
                    format!("{} more ", number_of_items_to_offer_randomly)
                },
                if number_of_items_to_offer_randomly == 1 { "" } else { "s" },
                probability_table.trim_end(),
            );

            while new_offered_item_ids.len() != (market_configuration.number_of_items as usize) {
                match commonness_of_items_that_can_be_offered
                    .choose_weighted(rng.deref_mut(), |(_, _, commonness)| *commonness)
                {
                    Ok((id, price, commonness)) => {
                        log::info!(
                            "offering randomly selected \"{}\" \
                            with {:.6}% probability ({} / {}) for {}",
                            id,
                            ((*commonness as f64) / (total_commonness as f64)) * 100.00,
                            commonness,
                            total_commonness,
                            price,
                        );
                        new_offered_item_ids.push((*id).clone())
                    },
                    Err(error) => {
                        log::error!(
                            "unable to choose a random item to offer in the market ({})",
                            error,
                        );
                        break;
                    },
                }
            }
        }
    }

    market_state.offered_item_ids = new_offered_item_ids;
    market_state.locked_item_indices = new_locked_item_indices;

    market_state.acquired_item_indices.clear();
    market_state.processed_acquirements = 0;

    log::info!("market is refreshed");
}

/// Calculates the commonness of an item in the item registry in the market configuration.
pub fn commonness_of(market_configuration: &MarketConfiguration, item: &RegisteredItem) -> u64 {
    let id = item.id();

    if market_configuration.blacklisted_items.contains(&id) {
        return 0;
    }

    if market_configuration.whitelisted_items.contains(&id) {
        return item.commonness;
    }

    if item.needs_to_be_whitelisted_to_appear_in_market() {
        return 0;
    }

    let mut is_included = market_configuration.whitelisted_items.is_empty();

    if !market_configuration.whitelisted_tags.is_empty() {
        let has_whitelisted_tag =
            item.tags.iter().any(|tag| market_configuration.whitelisted_tags.contains(tag));
        is_included = has_whitelisted_tag;
    }

    if !market_configuration.blacklisted_tags.is_empty() {
        let has_blacklisted_tag =
            item.tags.iter().any(|tag| market_configuration.blacklisted_tags.contains(tag));
        if has_blacklisted_tag {
            is_included = false;
        }
    }

    if is_included { item.commonness } else { 0 }
}


/// Deinitializes the market.
pub fn deinitialize_market(mut commands: Commands) {
    log::info!("deinitializing the market");
    commands.insert_resource(Balance::default());
    commands.insert_resource(ExperienceToBalanceRatio::default());
    commands.insert_resource(MarketConfiguration::default());
    commands.insert_resource(MarketState::default());
}
