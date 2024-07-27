pub mod input;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::combat::projectile::projectile;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Attack, AttackController)>();

    app.add_plugins(input::plugin);
}

/// Attack parameters.
#[derive(Component, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Component)]
#[serde(default)]
pub struct Attack {
    /// A multiplier for effects like damage and knockback.
    pub power: f32,
    /// A multiplier for initial projectile speed.
    pub force: f32,
    /// The color of the projectile.
    pub color: Color,
    /// The relative distance to spawn projectiles from.
    pub offset: f32,
    /// The key of the projectile to attack with.
    #[serde(rename = "projectile")]
    pub projectile_key: Option<String>,
}

impl Configure for Attack {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_attack.in_set(UpdateSet::Spawn));
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            power: 1.0,
            force: 1.0,
            color: Color::WHITE,
            offset: 5.0,
            projectile_key: None,
        }
    }
}

fn apply_attack(
    mut commands: Commands,
    attack_query: Query<(
        &Attack,
        &AttackController,
        &GlobalTransform,
        Option<&LinearVelocity>,
        &Faction,
    )>,
) {
    for (attack, controller, gt, velocity, &faction) in &attack_query {
        if !controller.fire || controller.aim == Vec2::ZERO {
            continue;
        }
        let projectile_key = c!(attack.projectile_key.as_ref());

        let translation = gt.translation();
        // Spawn projectile at an initial distance away from attacker.
        let pos = translation.xy() + attack.offset * controller.aim;
        // Render projectile above attacker.
        let translation = pos.extend(translation.z + 2.0);

        // Projectiles get a boost if the actor is moving in the same direction.
        let aligned_speed = velocity
            .filter(|v| v.0 != Vec2::ZERO)
            .map(|v| v.dot(controller.aim) / controller.aim.length())
            .unwrap_or(0.0)
            .clamp(0.0, 100.0);
        let speed_force_boost = 0.8;
        let speed_force = aligned_speed / 100.0 * speed_force_boost + 1.0;

        commands
            .spawn_with(projectile(
                projectile_key,
                faction,
                attack.power,
                attack.force * controller.aim * speed_force,
                attack.color,
            ))
            .insert(Transform::from_translation(translation));
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackController {
    pub aim: Vec2,
    pub fire: bool,
}

impl Configure for AttackController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, reset_attack_controller.in_set(UpdateSet::SyncEarly));
    }
}

fn reset_attack_controller(mut controller_query: Query<&mut AttackController>) {
    for mut controller in &mut controller_query {
        *controller = default();
    }
}
