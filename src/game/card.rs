use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::utils::HashMap;
use pyri_state::prelude::*;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(exit_playing, enter_playing),
    );
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum CardKey {
    Placeholder,
}

pub struct Card {
    pub name: String,
    pub description: String,
    pub action: SystemId<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct CardStorage(pub HashMap<CardKey, Card>);

// TODO: Move this into a sub-folder of storing different attacks?
fn basic_attack(In(entity): In<Entity>) {
    println!("Entity {} Attacked", entity);
}

fn enter_playing(world: &mut World) {
    let id = world.register_system(basic_attack);
    world.insert_resource(CardStorage(
        [(
            CardKey::Placeholder,
            Card {
                name: "Attack".to_string(),
                description: "Attack Description".to_string(),
                action: id,
            },
        )]
        .into_iter()
        .collect(),
    ));
}

fn exit_playing(mut _commands: Commands) {
    // TODO: despawn cards?
}