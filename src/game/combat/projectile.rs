use std::time::Duration;

use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::prelude::*;
use bevy_tweening::*;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::facing::Facing;
use crate::game::actor::faction::Faction;
use crate::game::card::attack::AimTowardsFacing;
use crate::game::card::attack::AttackOnBeat;
use crate::game::cleanup::DespawnOnHit;
use crate::game::cleanup::DespawnOnTimer;
use crate::game::cleanup::DespawnRadiusSq;
use crate::game::combat::damage::HitboxDamage;
use crate::game::combat::hit::Hitbox;
use crate::game::combat::knockback::HitboxKnockback;
use crate::game::GameLayer;
use crate::game::GameRoot;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ProjectileConfig>,
        DespawnOnHit,
        DespawnRadiusSq,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct ProjectileConfig {
    pub projectiles: HashMap<String, Projectile>,
}

impl Config for ProjectileConfig {
    const PATH: &'static str = "config/projectile.ron";
    const EXTENSION: &'static str = "projectile.ron";

    fn on_load(&mut self, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();

        for projectile in self.projectiles.values_mut() {
            projectile.texture = asset_server.load(&projectile.texture_path);
            if !projectile.spawn_sfx_path.is_empty() {
                projectile.spawn_sfx = Some(asset_server.load(&projectile.spawn_sfx_path));
            }
        }
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.projectiles
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
            && self.projectiles.values().all(|x| {
                !x.spawn_sfx
                    .as_ref()
                    .is_some_and(|x| !asset_server.is_loaded_with_dependencies(x))
            })
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct Projectile {
    pub name: String,

    #[serde(rename = "texture")]
    pub texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
    #[serde(rename = "spawn_sfx", default)]
    pub spawn_sfx_path: String,
    #[serde(skip)]
    pub spawn_sfx: Option<Handle<AudioSource>>,
    #[serde(default = "one")]
    pub spawn_sfx_volume: f64,

    /// Lifetime in seconds, not beats.
    pub lifetime: f32,
    /// Hitbox radius.
    pub radius: f32,
    pub speed: f32,
    pub damage: f32,
    pub knockback: f32,
}

fn one() -> f64 {
    1.0
}

const FADE_SECS: f32 = 0.2;

pub fn projectile(
    key: impl Into<String>,
    faction: Faction,
    power: f32,
    force: Vec2,
    color: impl Into<Color>,
    child_projectiles: Option<(Attack, Facing)>,
) -> impl EntityCommand {
    let key = key.into();
    let color = color.into();

    move |entity: Entity, world: &mut World| {
        let mut system_state =
            SystemState::<(ConfigRef<ProjectileConfig>, Res<GameRoot>, Res<Audio>)>::new(world);
        let (config, game_root, audio) = system_state.get(world);
        let config = r!(config.get());
        let projectile = r!(config.projectiles.get(&key)).clone();
        let parent = game_root.projectiles;
        let target_layers = {
            let mut x = LayerMask::ALL;
            // Projectiles cannot collide with each other.
            x.remove(GameLayer::Projectile);
            // Projectiles cannot collide with their owner's layer.
            x.remove(faction.layer());
            x
        };

        if let (Faction::Player, Some(spawn_sfx)) = (faction, projectile.spawn_sfx) {
            audio
                .play(spawn_sfx)
                .with_volume(projectile.spawn_sfx_volume);
        }

        let mut entity = world.entity_mut(entity);
        entity
            .insert((
                Name::new(projectile.name.replace(' ', "")),
                // Appearance:
                (
                    SpriteBundle {
                        sprite: Sprite { color, ..default() },
                        texture: projectile.texture.clone(),
                        ..default()
                    },
                    Animator::new(
                        Delay::new(Duration::from_secs_f32(
                            (projectile.lifetime - FADE_SECS).max(0.001),
                        ))
                        .then(Tween::new(
                            EaseMethod::Linear,
                            Duration::from_secs_f32(projectile.lifetime.clamp(0.001, FADE_SECS)),
                            lens::SpriteColorLens {
                                start: color,
                                end: Color::NONE,
                            },
                        )),
                    ),
                ),
                // Physics:
                (
                    RigidBody::Kinematic,
                    Collider::circle(projectile.radius),
                    CollisionLayers::new(GameLayer::Projectile, target_layers),
                    LockedAxes::ROTATION_LOCKED,
                    LinearVelocity(projectile.speed * force),
                ),
                // Combat:
                (
                    Hitbox,
                    HitboxDamage(power * projectile.damage),
                    HitboxKnockback(power * projectile.knockback),
                ),
                // TODO: Additional cleanup conditions that could be added: entity cap.
                // Cleanup:
                (
                    DespawnOnHit,
                    DespawnRadiusSq::new(200.0),
                    DespawnOnTimer(Timer::from_seconds(projectile.lifetime, TimerMode::Once)),
                ),
            ))
            .set_parent(parent);

        if let Some((attack, facing)) = child_projectiles {
            entity.insert((
                AttackOnBeat(attack.clone()),
                AimTowardsFacing,
                AttackController::default(),
                facing,
                faction,
                attack,
            ));
        }
    }
}
