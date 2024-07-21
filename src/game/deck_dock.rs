use bevy::prelude::*;
use bevy::render::texture::ImageLoaderSettings;
use bevy::render::texture::ImageSampler;
use leafwing_input_manager::common_conditions::action_just_pressed;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::game::deck::Deck;
use crate::screen::playing::PlayingAction;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<DeckDockConfig>>();
    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(exit_playing, enter_playing),
    );

    app.add_systems(
        Update,
        Screen::Playing.on_update((
            animate_move_towards,
            rotate_dock_left.run_if(action_just_pressed(PlayingAction::RotateDock)),
            create_dock.run_if(any_with_component::<DrawDeck>),
        )),
    );
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct DeckDockConfig {
    pub card_width: usize,
    pub max_dock_width: usize,
    pub max_dock_height: usize,
    pub minimum_card_distance: usize,
    pub dock_translation: Vec3,
    pub dock_scale: f32,
}

impl Config for DeckDockConfig {
    const PATH: &'static str = "config/deck_dock.ron";

    const EXTENSION: &'static str = "deck_dock.ron";

    fn on_load(&mut self, _: &mut World) {}
}

fn enter_playing() {}



#[derive(Component)]
struct MoveToward {
    start: Vec3,
    end: Vec3,
    duration: Timer,
}

fn animate_move_towards(
    mut commands: Commands,
    mut move_towards: Query<(Entity, &mut Transform, &mut MoveToward)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut move_toward) in &mut move_towards {
        move_toward.duration.tick(time.delta());
        transform.translation = move_toward
            .start
            .lerp(move_toward.end, move_toward.duration.fraction());

        if let (true, Some(mut e)) = (move_toward.duration.finished(), commands.get_entity(entity)) {
            e.remove::<MoveToward>();
        }
    }
}

fn exit_playing(mut commands: Commands, dock: Query<Entity, With<DeckDock>>) {
    for entity in &dock {
        if let Some(e) = commands.get_entity(entity) {
            e.despawn_recursive();
        }
    }
}

#[derive(Component)]
struct DeckDock {
    number_of_cards: usize,
}

impl DeckDock {
    // TODO: we might be able to get away with having the dock extend beyond the visible area and simplyify the logic here
    fn get_position(&self, config: &DeckDockConfig, index: usize) -> Option<Vec3> {
        let number_of_cards = self.number_of_cards;
        let DeckDockConfig {
            card_width,
            minimum_card_distance,
            max_dock_width,
            max_dock_height,
            ..
        } = *config;
        if index >= self.number_of_cards {
            return None;
        }

        let dock_width =
            number_of_cards * card_width + (number_of_cards - 1) * minimum_card_distance;
        let start_offset = (max_dock_width as isize - dock_width as isize) / 2;
        let x_position = start_offset
            + (index as isize * (card_width as isize + minimum_card_distance as isize));

        if x_position < 0 || x_position as usize > max_dock_width {
            return None;
        }

        let center_index = if number_of_cards % 2 == 0 {
            number_of_cards / 2 - 1
        } else {
            number_of_cards / 2
        };

        let height_step = if max_dock_height < number_of_cards {
            1
        } else {
            max_dock_height / number_of_cards
        };

        let y_position = if index <= center_index {
            max_dock_height - (center_index - index) * height_step
        } else {
            max_dock_height - (index - center_index) * height_step
        };

        Some(Vec3::new(
            x_position as f32 - (max_dock_width / 2) as f32,
            y_position as f32 - (max_dock_height / 2) as f32,
            0.0,
        ))
    }
}

fn rotate_dock_left(
    mut commands: Commands,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    docks: Query<(&DeckDock, &Children)>,
    mut cards: Query<(Entity, &Transform, &mut CardPosition)>,
) {
    let config = r!(config.get(&config_handle.0));
    for (dock, children) in &docks {
        for child in children.iter() {
            if let Ok((entity, transform, mut card_position)) = cards.get_mut(*child) {
                **card_position = if **card_position == 0 {
                    dock.number_of_cards - 1
                } else {
                    **card_position - 1
                };

                if let Some(target) = dock.get_position(config, **card_position) {
                    commands.entity(entity).insert((
                        MoveToward {
                            start: transform.translation,
                            end: target,
                            duration: Timer::from_seconds(0.5, TimerMode::Once),
                        },
                        Visibility::Visible,
                    ));
                } else {
                    commands.entity(entity).insert(Visibility::Hidden);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct DrawDeck;

#[derive(Component, Deref, DerefMut)]
struct CardPosition(usize);

fn create_dock(
    mut commands: Commands,
    config_handle: Res<ConfigHandle<DeckDockConfig>>,
    config: Res<Assets<DeckDockConfig>>,
    decks: Query<(Entity, &Deck), With<DrawDeck>>,
    asset_server: Res<AssetServer>,
) {
    let config = r!(config.get(&config_handle.0));
    for (entity, deck) in &decks {
        let number_of_cards = deck.cards.iter().len();
        commands
            .spawn((
                DeckDock {
                    number_of_cards,
                },
                SpatialBundle {
                    transform: Transform::from_translation(config.dock_translation)
                        .with_scale(Vec3::splat(config.dock_scale)),
                    ..default()
                },
                Name::new("DeckDock"),
            ))
            .with_children(|builder| {
                for (i, _) in deck.cards.iter().enumerate() {
                    create_card(builder, &asset_server, i);
                }
            });

        if let Some(mut e) = commands.get_entity(entity) {
            e.remove::<DrawDeck>();
        }
    }
}

fn create_card(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    index: usize,
) -> Entity {
    builder
        .spawn((
            Name::new("SampleCard"),
            CardPosition(index),
            SpriteBundle {
                texture: asset_server.load_with_settings(
                    "embedded://bevy_jam_5/game/cards/sample_card.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::linear();
                    },
                ),
                ..default()
            },
        ))
        .id()
}