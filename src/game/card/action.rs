use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::card::attack::AimTowardsFacing;
use crate::game::card::attack::DoubleBeat;
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
                    CardActionKey::Rest,
                    world.register_system(|_: In<Entity>, _: &mut World| {}),
                ),
                (
                    CardActionKey::Step,
                    world.register_system(|In(entity): In<Entity>, world: &mut World| {
                        r!(world.get_entity_mut(entity))
                            .insert(RemoveOnBeat::bundle(MoveTowardsFacing, 2));
                    }),
                ),
                (
                    CardActionKey::DoubleBeat,
                    world.register_system(|In(entity): In<Entity>, world: &mut World| {
                        r!(world.get_entity_mut(entity)).insert((
                            RemoveOnBeat::bundle(DoubleBeat, 2),
                            RemoveOnBeat::bundle(AimTowardsFacing, 2),
                        ));
                    }),
                ),
            ]
            .into_iter()
            .map(|(key, sys)| (key, CardAction(sys)))
            .collect(),
        )
    }
}

#[derive(Reflect, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub enum CardActionKey {
    #[default]
    Rest,
    Step,
    DoubleBeat,
}

/// A newtyped `SystemId<Entity>` with a `Default` impl.
#[derive(Reflect, Copy, Clone)]
#[reflect(Default)]
pub struct CardAction(#[reflect(ignore)] pub SystemId<Entity>);

impl Default for CardAction {
    fn default() -> Self {
        Self(SystemId::from_entity(Entity::PLACEHOLDER))
    }
}
