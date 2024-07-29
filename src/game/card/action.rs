use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::attack::Attack;
use crate::game::actor::health::Health;
use crate::game::actor::player::IsPlayer;
use crate::game::card::attack::AimTowardsFacing;
use crate::game::card::attack::AttackOnBeat;
use crate::game::card::movement::MoveTowardsFacing;
use crate::game::cleanup::RemoveOnBeat;
use crate::game::cleanup::RemoveOnTimer;
use crate::game::combat::damage::HitboxDamage;
use crate::game::combat::hit::Immune;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<CardActionMap>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CardActionMap(pub HashMap<CardActionKey, CardAction>);

impl Configure for CardActionMap {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for CardActionMap {
    fn from_world(world: &mut World) -> Self {
        Self(
            [
                (
                    CardActionKey::Step,
                    world.register_system(
                        |In((entity, modifier)): In<(Entity, CardActionModifier)>,
                         world: &mut World| {
                            let mut entity = r!(world.get_entity_mut(entity));
                            entity.insert(RemoveOnBeat::bundle(
                                MoveTowardsFacing,
                                modifier.remove_on_beat,
                            ));

                            // Player actor has extra benefits
                            if entity.contains::<IsPlayer>() {
                                entity.insert(RemoveOnTimer::bundle(
                                    HitboxDamage(modifier.hitbox_damage.0),
                                    Timer::from_seconds(modifier.hitbox_damage.1, TimerMode::Once),
                                ));

                                if let Some(timer) = modifier.immunity {
                                    entity.insert(RemoveOnTimer::bundle(
                                        Immune,
                                        Timer::from_seconds(timer, TimerMode::Once),
                                    ));
                                }
                            }
                        },
                    ),
                ),
                (
                    CardActionKey::Attack,
                    world.register_system(
                        |In((entity, modifier)): In<(Entity, CardActionModifier)>,
                         world: &mut World| {
                            r!(world.get_entity_mut(entity)).insert((
                                RemoveOnBeat::bundle(
                                    AttackOnBeat(modifier.attack.clone()),
                                    modifier.remove_on_beat,
                                ),
                                RemoveOnBeat::bundle(AimTowardsFacing, modifier.remove_on_beat),
                            ));
                        },
                    ),
                ),
                (
                    CardActionKey::Heal,
                    world.register_system(
                        |In((entity, modifier)): In<(Entity, CardActionModifier)>,
                         world: &mut World| {
                            let mut entity = r!(world.get_entity_mut(entity));
                            let mut health = r!(entity.get_mut::<Health>());
                            health.current += modifier.heal_flat;
                            health.current += modifier.heal_percent / 100.0 * health.max;
                        },
                    ),
                ),
            ]
            .into_iter()
            .map(|(key, sys)| (key, CardAction(sys)))
            .collect(),
        )
    }
}

#[derive(Reflect, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CardActionKey {
    Step,
    Attack,
    Heal,
}

/// A newtyped `SystemId` with a `Default` impl.
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CardAction(#[reflect(ignore)] pub SystemId<(Entity, CardActionModifier)>);

impl Default for CardAction {
    fn default() -> Self {
        Self(SystemId::from_entity(Entity::PLACEHOLDER))
    }
}

#[derive(Default, Reflect, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CardActionModifier {
    /// Remove component after this many eighth-beats.
    remove_on_beat: usize,
    /// Remove component when this timer finishes.
    remove_on_timer: Timer,
    attack: Attack,
    immunity: Option<f32>,
    hitbox_damage: (f32, f32), // damage, time
    heal_percent: f32,
    heal_flat: f32,
}
