use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::game::combat::damage::HitboxDamage;
use crate::game::combat::hit::Hitbox;
use crate::game::combat::hit::OnHit;
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

    pub radius: f32,
    pub speed: f32,
    pub damage: f32,
    pub knockback: f32,
}

pub fn projectile(key: impl Into<String>, power: f32, force: Vec2) -> impl EntityCommand<World> {
    let key = key.into();

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
                // TODO: Additional cleanup conditions that could be added: lifetime, entity cap.
                // Cleanup:
                (DespawnOnHit, DespawnOnDistanceSq(200.0 * 200.0)),
            ))
            .set_parent(parent);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DespawnOnHit;

impl Configure for DespawnOnHit {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(despawn_on_hit);
    }
}

fn despawn_on_hit(
    trigger: Trigger<OnHit>,
    mut despawn: ResMut<DespawnSet>,
    despawn_query: Query<(), With<DespawnOnHit>>,
) {
    let hitbox = trigger.event().0;
    if despawn_query.contains(hitbox) {
        despawn.recursive(hitbox);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct DespawnOnDistanceSq(f32);

impl Configure for DespawnOnDistanceSq {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, despawn_on_distance_sq.in_set(UpdateSet::Update));
    }
}

fn despawn_on_distance_sq(
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    mut despawn: ResMut<DespawnSet>,
    despawn_query: Query<(Entity, &GlobalTransform, &DespawnOnDistanceSq)>,
) {
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let camera_pos = camera_gt.translation().xy();

    for (entity, gt, limit) in &despawn_query {
        if gt.translation().xy().distance_squared(camera_pos) >= limit.0 {
            despawn.recursive(entity);
        }
    }
}
