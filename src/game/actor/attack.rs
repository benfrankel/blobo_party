pub mod input;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::actor::faction::Faction;
use crate::game::combat::projectile::projectile;
use crate::game::GameLayer;
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
    pub projectile: Option<String>,
}

impl Configure for Attack {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_attack.in_set(UpdateSet::Update));
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self {
            power: 1.0,
            force: 1.0,
            color: Color::WHITE,
            offset: 7.0,
            projectile: None,
        }
    }
}

fn apply_attack(
    mut commands: Commands,
    attack_query: Query<(&Attack, &AttackController, &GlobalTransform, &Faction)>,
) {
    for (attack, controller, gt, faction) in &attack_query {
        if controller.0 == Vec2::ZERO {
            continue;
        }
        let projectile_key = c!(attack.projectile.as_ref());

        let translation = gt.translation();
        // Spawn projectile at an initial distance away from attacker.
        let pos = translation.xy() + attack.offset * controller.0;
        // Render projectile above attacker.
        let translation = pos.extend(translation.z + 2.0);

        let mut target_layers = LayerMask::ALL;
        // Projectiles cannot collide with each other.
        target_layers.remove(GameLayer::Projectile);
        // Projectiles cannot collide with their owner's layer.
        target_layers.remove(faction.layer());

        commands
            .spawn_with(projectile(
                projectile_key,
                attack.power,
                attack.force * controller.0,
                attack.color,
            ))
            .insert((
                Transform::from_translation(translation),
                CollisionLayers::new(GameLayer::Projectile, target_layers),
            ));
    }
}

// TODO: Create a component that aims towards facing (and a component that moves towards facing).
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AttackController(pub Vec2);

impl Configure for AttackController {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, reset_attack_controller.in_set(UpdateSet::SyncEarly));
    }
}

fn reset_attack_controller(mut controller_query: Query<&mut AttackController>) {
    for mut controller in &mut controller_query {
        controller.0 = Vec2::ZERO;
    }
}
