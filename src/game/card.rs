use bevy::asset::embedded_asset;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use pyri_state::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // TODO: setup correct asset loading for cards
    embedded_asset!(app, "cards/sample_card.png");

    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(exit_playing, enter_playing),
    );
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum CardKey {
    Placeholder,
}

// TODO: Remove this `allow` later.
#[allow(dead_code)]
pub struct Card {
    pub path: String,
    pub action: SystemId<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CardStorage(pub HashMap<CardKey, Card>);

// TODO: Move this into a sub-folder of storing different attacks?
fn basic_attack(In(_): In<Entity>) {}

fn enter_playing(world: &mut World) {
    let id = world.register_system(basic_attack);
    world.insert_resource(CardStorage(
        [(
            CardKey::Placeholder,
            Card {
                path: "embedded://blobo_party/game/cards/sample_card.png".to_string(),
                action: id,
            },
        )]
        .into(),
    ));
}

fn exit_playing(mut _commands: Commands) {
    // TODO: despawn cards?
}
