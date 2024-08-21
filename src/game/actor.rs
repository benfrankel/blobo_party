pub mod attack;
pub mod enemy;
pub mod facing;
pub mod faction;
pub mod health;
pub mod level;
pub mod movement;
pub mod player;
mod shield;

use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::utils::HashMap;
use iyes_progress::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use shield::IsShield;

use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::actor::health::Health;
use crate::game::actor::health::HealthBar;
use crate::game::actor::level::xp::Xp;
use crate::game::actor::level::xp::XpReward;
use crate::game::actor::level::Level;
use crate::game::actor::movement::Movement;
use crate::game::actor::movement::MovementController;
use crate::game::actor::movement::OldMovementController;
use crate::game::audio::music::Beat;
use crate::game::card::deck::Deck;
use crate::game::combat::hit::Hurtbox;
use crate::game::sprite::SpriteAnimation;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<ActorConfig>>();

    app.add_plugins((
        attack::plugin,
        enemy::plugin,
        facing::plugin,
        faction::plugin,
        health::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        shield::plugin,
    ));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct ActorConfig {
    pub players: HashMap<String, Actor>,
    pub enemies: HashMap<String, Actor>,
}

impl Config for ActorConfig {
    const PATH: &'static str = "config/actor.ron";
    const EXTENSION: &'static str = "actor.ron";

    fn on_load(&mut self, world: &mut World) {
        let mut system_state =
            SystemState::<(Res<AssetServer>, ResMut<Assets<TextureAtlasLayout>>)>::new(world);
        let (asset_server, mut layouts) = system_state.get_mut(world);

        for actor in self.players.values_mut().chain(self.enemies.values_mut()) {
            actor.texture = asset_server.load(&actor.texture_path);
            actor.texture_atlas_layout = layouts.add(&actor.texture_atlas_grid);
            actor.sprite_animation.calculate_total_beats();
        }
    }

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = true.into();

        for actor in self.players.values().chain(self.enemies.values()) {
            progress += asset_server
                .is_loaded_with_dependencies(&actor.texture)
                .into();
        }

        progress
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct Actor {
    pub name: String,
    /// The earliest level this actor can spawn as an enemy.
    #[serde(default)]
    pub min_level: usize,
    /// The latest level this actor can spawn as an enemy.
    #[serde(default = "inf")]
    pub max_level: usize,
    /// The relative probability of this actor spawning as an enemy.
    #[serde(default = "one")]
    pub weight: f64,

    #[serde(rename = "texture")]
    texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
    texture_atlas_grid: TextureAtlasGrid,
    #[serde(skip)]
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub sprite_animation: SpriteAnimation,

    #[serde(default)]
    pub movement: Movement,
    #[serde(default)]
    pub attack: Attack,
    #[serde(default)]
    pub health: Health,
    #[serde(default)]
    pub xp_reward: XpReward,
    #[serde(default)]
    pub deck: Deck,
}

fn inf() -> usize {
    usize::MAX
}

fn one() -> f64 {
    1.0
}

impl EntityCommand for Actor {
    fn apply(mut self, id: Entity, world: &mut World) {
        self.deck.active += world.resource::<Beat>().total as isize - 1;
        let bubble_texture = world.resource::<PlayingAssets>().bubble.clone();

        world
            .entity_mut(id)
            .insert((
                Name::new(self.name.replace(' ', "")),
                // Appearance:
                (
                    SpriteBundle {
                        texture: self.texture,
                        ..default()
                    },
                    TextureAtlas {
                        layout: self.texture_atlas_layout,
                        index: 0,
                    },
                    self.sprite_animation,
                    Facing::default(),
                ),
                // Physics:
                (
                    RigidBody::Dynamic,
                    Collider::circle(4.0),
                    LockedAxes::ROTATION_LOCKED,
                    self.movement,
                    MovementController::default(),
                    OldMovementController::default(),
                ),
                // Combat:
                (
                    self.attack,
                    AttackController::default(),
                    self.health,
                    Hurtbox,
                    // TODO: Death animation.
                ),
                // Inventory:
                (Level::default(), Xp::default(), self.xp_reward, self.deck),
            ))
            .with_children(|children| {
                children
                    .spawn_with(HealthBar {
                        size: vec2(8.0, 1.0),
                    })
                    .insert(Transform::from_translation(vec3(0.0, -4.5, 1.0)));
                children
                    .spawn((
                        Name::new("Shield"),
                        SpriteBundle {
                            transform: Transform::default(),
                            texture: bubble_texture,
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        IsShield,
                    ))
                    .insert(Transform::from_translation(vec3(0.0, -0.5, 2.0)));
            });
    }
}
