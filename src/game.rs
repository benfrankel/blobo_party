//! Game mechanics and content

pub mod actor;
mod card;
mod deck;
mod deck_dock;
pub mod sprite;
pub mod step;

use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GameRoot>();

    app.add_plugins((
        actor::plugin,
        sprite::plugin,
        step::plugin,
        card::plugin,
        deck::plugin,
        deck_dock::plugin,
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameRoot {
    pub game: Entity,
}

impl Configure for GameRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for GameRoot {
    fn from_world(world: &mut World) -> Self {
        let game = world
            .spawn((Name::new("Game"), SpatialBundle::default()))
            .id();

        Self { game }
    }
}
