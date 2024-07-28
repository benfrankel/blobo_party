use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::attack::Attack;
use crate::game::actor::health::Health;
use crate::game::card::attack::AimTowardsFacing;
use crate::game::card::attack::AttackOnBeat;
use crate::game::card::movement::MoveTowardsFacing;
use crate::game::cleanup::RemoveOnBeat;
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
                            r!(world.get_entity_mut(entity)).insert(RemoveOnBeat::bundle(
                                MoveTowardsFacing,
                                modifier.remove_on_beat,
                            ));
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
    heal_percent: f32,
    heal_flat: f32,
}
