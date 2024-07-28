pub mod input;

use std::f32::consts::TAU;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::facing::Facing;
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
    /// Optional list of facing offsets for multiple shots.
    #[serde(default)]
    pub multi_shot: Option<MultiShot>,
    #[serde(default)]
    pub child_projectile: Option<ChildProjectile>,
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
            multi_shot: None,
            child_projectile: None,
        }
    }
}

// Way to specify a projectile fires multiple shots, one for each offset.
#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct MultiShot(pub Vec<f32>);

#[derive(Reflect, Serialize, Deserialize, Clone, Debug)]
pub struct ChildProjectile {
    // The direction child projectiles will go. The first entry is relative to the parent projectile's velocity and any additional
    // entries are treated as multishots of the first entry, so their directions are relative to the first projectile.
    // i.e., [0.25, 0.5] would fire one shot to the left of the parent and one shot to the right because 0.5 is "behind" relative to the first child.
    pub offsets: Vec<f32>,
    /// The key of the child projectiles.
    #[serde(rename = "projectile")]
    pub projectile_key: Option<String>,
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

        // Handle creating multiple shots based off controller's aim
        let mut shots = vec![controller.aim];
        if let Some(additional_shots) = &attack.multi_shot {
            for offset in additional_shots.0.iter() {
                let shot_angle = Vec2::from_angle(offset * TAU);
                shots.push(controller.aim.rotate(shot_angle));
            }
        }

        // Setup any child projectiles that the current attack's projectile will create
        let child_projectiles = if let Some(child) = &attack.child_projectile {
            if let Some((primary, multi_shots)) = child.offsets.split_first() {
                let facing_direction = controller.aim.rotate(Vec2::from_angle(primary * TAU));
                let mut child_attack = attack.clone();
                child_attack.child_projectile = None;
                child_attack.multi_shot = Some(MultiShot((*multi_shots).into()));
                child_attack.projectile_key = child.projectile_key.clone();

                Some((child_attack, Facing(c!(Dir2::new(facing_direction)))))
            } else {
                None
            }
        } else {
            None
        };

        for shot in shots.iter() {
            // Projectiles get a boost if the actor is moving in the same direction.
            let aligned_speed = velocity
                .filter(|v| v.0 != Vec2::ZERO)
                .map(|v| v.dot(*shot) / shot.length())
                .unwrap_or(0.0)
                .clamp(0.0, 100.0);
            let speed_force_boost = 0.8;
            let speed_force = aligned_speed / 100.0 * speed_force_boost + 1.0;

            commands
                .spawn_with(projectile(
                    projectile_key,
                    faction,
                    attack.power,
                    attack.force * *shot * speed_force,
                    attack.color,
                    child_projectiles.clone(),
                ))
                .insert(Transform::from_translation(translation));
        }
    }
}

#[derive(Component, Reflect, Default, Clone)]
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
