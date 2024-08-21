use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::attack::Attack;
use crate::game::actor::attack::AttackController;
use crate::game::actor::health::Health;
use crate::game::actor::movement::Movement;
use crate::game::actor::player::IsPlayer;
use crate::game::audio::music::Beat;
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
                    CardActionKey::Move,
                    world.register_system(
                        |In((entity, modifier)): In<(Entity, CardActionModifier)>,
                         world: &mut World| {
                            let mut entity = r!(world.get_entity_mut(entity));
                            entity.insert(RemoveOnBeat::bundle(
                                MoveTowardsFacing(modifier.movement),
                                modifier.remove_on_beat,
                            ));

                            // Player actor has extra benefits:
                            if entity.contains::<IsPlayer>() {
                                if modifier.contact_damage > 0.0 {
                                    entity.insert(RemoveOnBeat::bundle(
                                        HitboxDamage(modifier.contact_damage),
                                        modifier.contact_beats,
                                    ));
                                }

                                if modifier.immunity > 0.0 {
                                    entity.insert(RemoveOnTimer::bundle(
                                        Immune,
                                        Timer::from_seconds(modifier.immunity, TimerMode::Once),
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
                            let beat = world.resource::<Beat>().total;
                            r!(world.get_entity_mut(entity)).insert((
                                RemoveOnBeat::bundle(
                                    AttackOnBeat(
                                        modifier.attack.clone(),
                                        modifier.attack_on_beat,
                                        beat % modifier.attack_on_beat,
                                    ),
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

                            let mut attack_controller = r!(entity.get_mut::<AttackController>());
                            attack_controller.aim = Vec2::Y;
                            attack_controller.fire = true;

                            let mut attack = r!(entity.get_mut::<Attack>());
                            attack.projectile_key = modifier.attack.projectile_key.clone();
                            attack.offset = modifier.attack.offset;

                            // Player actor has extra benefits:
                            if entity.contains::<IsPlayer>() {
                                if modifier.contact_damage > 0.0 {
                                    entity.insert(RemoveOnBeat::bundle(
                                        HitboxDamage(modifier.contact_damage),
                                        modifier.contact_beats,
                                    ));
                                }

                                if modifier.immunity > 0.0 {
                                    entity.insert(RemoveOnTimer::bundle(
                                        Immune,
                                        Timer::from_seconds(modifier.immunity, TimerMode::Once),
                                    ));
                                }
                            }
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
    Move,
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

#[derive(Reflect, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct CardActionModifier {
    /// Remove component after this many eighth-beats.
    remove_on_beat: usize,
    /// Remove component when this timer finishes.
    remove_on_timer: Timer,
    attack: Attack,
    attack_on_beat: usize,
    movement: Movement,
    contact_damage: f32,
    contact_beats: usize,
    heal_percent: f32,
    heal_flat: f32,
    immunity: f32,
}

impl Default for CardActionModifier {
    fn default() -> Self {
        Self {
            remove_on_beat: 0,
            remove_on_timer: Timer::default(),
            attack: Attack::default(),
            attack_on_beat: 4,
            movement: Movement::default(),
            contact_damage: 0.0,
            contact_beats: 0,
            heal_percent: 0.0,
            heal_flat: 0.0,
            immunity: 0.0,
        }
    }
}
