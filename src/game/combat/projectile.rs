use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::cleanup::DespawnOnDistanceSq;
use crate::game::cleanup::DespawnOnHit;
use crate::game::cleanup::DespawnOnTimer;
use crate::game::combat::damage::HitboxDamage;
use crate::game::combat::hit::Hitbox;
use crate::game::combat::knockback::HitboxKnockback;
use crate::game::GameRoot;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        ConfigHandle<ProjectileConfig>,
        DespawnOnHit,
        DespawnOnDistanceSq,
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
        }
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.projectiles
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Projectile {
    pub name: String,

    pub texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,

    /// In seconds, not beats.
    pub lifetime: f32,
    pub radius: f32,
    pub speed: f32,
    pub damage: f32,
    pub knockback: f32,
}

pub fn projectile(
    key: impl Into<String>,
    power: f32,
    force: Vec2,
    color: impl Into<Color>,
) -> impl EntityCommand<World> {
    let key = key.into();
    let color = color.into();

    move |mut entity: EntityWorldMut| {
        let config_handle = entity.world().resource::<ConfigHandle<ProjectileConfig>>();
        let config = r!(entity
            .world()
            .resource::<Assets<ProjectileConfig>>()
            .get(&config_handle.0));
        let projectile = r!(config.projectiles.get(&key));
        let parent = entity.world().resource::<GameRoot>().projectiles;

        entity
            .insert((
                Name::new(projectile.name.replace(' ', "")),
                // Appearance:
                SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    texture: projectile.texture.clone(),
                    ..default()
                },
                // Physics:
                (
                    RigidBody::Kinematic,
                    Collider::circle(projectile.radius),
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
                    DespawnOnDistanceSq::new(200.0),
                    DespawnOnTimer(Timer::from_seconds(projectile.lifetime, TimerMode::Once)),
                ),
            ))
            .set_parent(parent);
    }
}
