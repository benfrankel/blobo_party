pub mod attack;
pub mod enemy;
pub mod facing;
pub mod faction;
pub mod health;
pub mod movement;
pub mod player;

use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::math::vec2;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::actor::health::Health;
use crate::game::actor::health::HealthBar;
use crate::game::actor::movement::Movement;
use crate::game::actor::movement::MovementController;
use crate::game::combat::death::DespawnOnDeath;
use crate::game::combat::hit::Hurtbox;
use crate::game::deck::create_deck;
use crate::game::sprite::SpriteAnimation;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<ActorConfig>>();

    app.add_plugins((
        attack::plugin,
        enemy::plugin,
        facing::plugin,
        faction::plugin,
        health::plugin,
        movement::plugin,
        player::plugin,
    ));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct ActorConfig {
    pub player: String,
    pub player_health_multiplier: f32,
    pub player_attack_power_multiplier: f32,
    pub player_attack_force_multiplier: f32,

    pub actors: HashMap<String, Actor>,
}

impl Config for ActorConfig {
    const PATH: &'static str = "config/actor.ron";
    const EXTENSION: &'static str = "actor.ron";

    fn on_load(&mut self, world: &mut World) {
        let mut system_state =
            SystemState::<(Res<AssetServer>, ResMut<Assets<TextureAtlasLayout>>)>::new(world);
        let (asset_server, mut layouts) = system_state.get_mut(world);

        for actor in self.actors.values_mut() {
            actor.texture = asset_server.load(&actor.texture_path);
            actor.texture_atlas_layout = layouts.add(&actor.texture_atlas_grid);
            actor.sprite_animation.calculate_total_steps();
        }
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.actors
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Actor {
    pub name: String,

    pub texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
    pub texture_atlas_grid: TextureAtlasGrid,
    #[serde(skip)]
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    pub sprite_animation: SpriteAnimation,

    pub movement: Movement,
    pub attack: Attack,
    pub health: f32,
}

fn actor_helper(mut entity: EntityWorldMut, key: Option<String>) -> EntityWorldMut {
    let config_handle = entity.world().resource::<ConfigHandle<ActorConfig>>();
    let config = r!(
        entity,
        entity
            .world()
            .resource::<Assets<ActorConfig>>()
            .get(&config_handle.0),
    );
    let actor = r!(
        entity,
        config.actors.get(key.as_ref().unwrap_or(&config.player)),
    );
    let mut attack = actor.attack.clone();
    let mut health = actor.health;
    if key.is_none() {
        attack.power *= config.player_attack_power_multiplier;
        attack.force *= config.player_attack_force_multiplier;
        health *= config.player_health_multiplier;
    }

    entity
        .insert((
            Name::new(actor.name.replace(' ', "")),
            // Appearance:
            (
                SpriteBundle {
                    texture: actor.texture.clone(),
                    ..default()
                },
                TextureAtlas {
                    layout: actor.texture_atlas_layout.clone(),
                    index: 0,
                },
                actor.sprite_animation.clone(),
                Facing::default(),
            ),
            // Physics:
            (
                RigidBody::Dynamic,
                Collider::circle(4.0),
                LockedAxes::ROTATION_LOCKED,
                actor.movement,
                MovementController::default(),
            ),
            // Combat:
            (
                attack,
                AttackController::default(),
                Health::new(health),
                Hurtbox,
                // TODO: Death animation instead, despawn when it's finished.
                DespawnOnDeath,
            ),
        ))
        .add(create_deck)
        .with_children(|children| {
            children
                .spawn_with(HealthBar {
                    size: vec2(8.0, 1.0),
                })
                .insert(Transform::from_translation(vec3(0.0, -4.5, 1.0)));
        });

    entity
}

pub fn actor(key: impl Into<String>) -> impl EntityCommand<World> {
    let key = key.into();
    move |entity: EntityWorldMut| {
        actor_helper(entity, Some(key));
    }
}
