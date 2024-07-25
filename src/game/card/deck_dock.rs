use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use pyri_state::prelude::*;

use crate::game::actor::player::IsPlayer;
use crate::game::actor::ActorConfig;
use crate::game::card::deck::Deck;
use crate::game::card::AddCardEvent;
use crate::game::card::CardConfig;
use crate::screen::playing::PlayingAction;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SelectedIndex>();

    app.add_systems(
        Update,
        Screen::Playing.on_update((
            handle_added_cards,
            highlight_selected,
            set_selected_from_player_deck,
            // TODO: Run these during level up menu.
            swap_card_left.run_if(action_just_pressed(PlayingAction::SwapCardLeft)),
            swap_card_right.run_if(action_just_pressed(PlayingAction::SwapCardRight)),
            select_left.run_if(action_just_pressed(PlayingAction::SelectCardLeft)),
            select_right.run_if(action_just_pressed(PlayingAction::SelectCardRight)),
            // TODO: Run this when exiting level up menu.
            sync_to_player_deck.run_if(action_just_pressed(PlayingAction::AcceptDeckChanges)),
        )),
    );
}

#[derive(Default, Resource, Deref, DerefMut)]
struct SelectedIndex(usize);

#[derive(Component)]
struct IsDeckDock;

#[derive(Component)]
struct VisualCard(String);

fn set_selected_from_player_deck(
    player_deck: Query<&Deck, With<IsPlayer>>,
    mut selected_index: ResMut<SelectedIndex>,
) {
    if let Some(deck) = player_deck.iter().last() {
        selected_index.0 = deck.selected;
    }
}

fn highlight_selected(
    deck_dock_query: Query<&Children, With<IsDeckDock>>,
    selected_index: Res<SelectedIndex>,
    mut card_query: Query<&mut TextureAtlas, With<VisualCard>>,
) {
    for children in &deck_dock_query {
        for (i, &child) in children.iter().enumerate() {
            let mut atlas = c!(card_query.get_mut(child));
            atlas.index = if i == selected_index.0 { 1 } else { 0 };
        }
    }
}

fn handle_added_cards(
    mut commands: Commands,
    mut add_card_events: EventReader<AddCardEvent>,
    dock_query: Query<Entity, With<IsDeckDock>>,
) {
    for event in add_card_events.read() {
        for dock in &dock_query {
            commands.spawn_with(card(event.0.clone())).set_parent(dock);
        }
    }
}

fn swap_card_left(
    mut selected_index: ResMut<SelectedIndex>,
    mut deck_dock: Query<&mut Children, With<IsDeckDock>>,
) {
    if selected_index.0 == 0 {
        // TODO: Wrap around.
        return;
    }

    let Some(mut children) = deck_dock.iter_mut().last() else {
        return;
    };

    children.swap(selected_index.0, selected_index.0 - 1);
    selected_index.0 = selected_index.saturating_sub(1);
}

fn swap_card_right(
    mut selected_index: ResMut<SelectedIndex>,
    mut deck_dock: Query<&mut Children, With<IsDeckDock>>,
) {
    let Some(mut children) = deck_dock.iter_mut().last() else {
        return;
    };
    if selected_index.0 >= children.len() - 1 {
        // TODO: play a bad sound
        return;
    }

    children.swap(selected_index.0, selected_index.0 + 1);
    selected_index.0 = selected_index.saturating_add(1);
}

fn select_left(mut selected_index: ResMut<SelectedIndex>) {
    selected_index.0 = selected_index.saturating_sub(1);
}

fn select_right(
    mut selected_index: ResMut<SelectedIndex>,
    deck_dock: Query<&Children, With<IsDeckDock>>,
) {
    let Some(children) = deck_dock.iter().last() else {
        return;
    };

    selected_index.0 = selected_index.saturating_add(1).min(children.len() - 1);
}

fn sync_to_player_deck(
    mut player_deck_query: Query<&mut Deck, With<IsPlayer>>,
    deck_dock_query: Query<&Children, With<IsDeckDock>>,
    visual_card_query: Query<&VisualCard>,
) {
    let children = r!(deck_dock_query.iter().last());

    let cards = children
        .iter()
        .filter_map(|child| visual_card_query.get(*child).ok())
        .map(|card| card.0.to_owned())
        .collect::<Vec<_>>();

    for mut deck in &mut player_deck_query {
        *deck = Deck::new(cards.clone())
    }
}

pub fn deck_dock(player_key: impl Into<String>) -> impl EntityCommand {
    let player_key = player_key.into();

    move |entity: Entity, world: &mut World| {
        let (config, actor_config) =
            SystemState::<(ConfigRef<CardConfig>, ConfigRef<ActorConfig>)>::new(world).get(world);
        let &CardConfig {
            deck_height,
            deck_column_gap,
            ..
        } = r!(config.get());
        let actor_config = r!(actor_config.get());
        let actor = r!(actor_config.players.get(&player_key)).clone();

        world
            .entity_mut(entity)
            .insert((
                Name::new("DeckDock"),
                NodeBundle {
                    style: Style {
                        width: Percent(100.0),
                        height: deck_height,
                        justify_content: JustifyContent::Center,
                        column_gap: deck_column_gap,
                        ..default()
                    },
                    ..default()
                },
                IsDeckDock,
            ))
            .with_children(|children| {
                actor.deck.cards.into_iter().for_each(|card_key| {
                    children.spawn_with(card(card_key));
                });
            });
    }
}

fn card(card_key: impl Into<String>) -> impl EntityCommand {
    let card_key = card_key.into();

    move |entity: Entity, world: &mut World| {
        let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
        let config = r!(config.get());
        let card = r!(config.card_map.get(&card_key));
        let background = r!(config.card_background_map.get(&card.background));
        let background_texture = background.texture.clone();
        let background_layout = background.texture_atlas_layout.clone();
        let icon_texture = card.icon_texture.clone();

        world
            .entity_mut(entity)
            .insert((
                Name::new("Card"),
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    image: UiImage::new(background_texture),
                    ..default()
                },
                TextureAtlas {
                    layout: background_layout,
                    index: 0,
                },
                VisualCard(card_key),
            ))
            .with_children(|children| {
                children.spawn((
                    Name::new("CardIcon"),
                    ImageBundle {
                        style: Style {
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        transform: Transform::from_scale(Vec3::splat(0.5)),
                        image: UiImage::new(icon_texture),
                        ..default()
                    },
                ));
            });
    }
}
