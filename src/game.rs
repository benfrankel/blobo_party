//! Game mechanics and content

pub mod actor;
mod card;
pub mod combat;
mod deck;
mod deck_dock;
pub mod projectile;
pub mod sprite;
pub mod step;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GameRoot>();

    app.add_plugins((
        actor::plugin,
        card::plugin,
        combat::plugin,
        deck::plugin,
        deck_dock::plugin,
        projectile::plugin,
        sprite::plugin,
        step::plugin,
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameRoot {
    pub players: Entity,
    pub enemies: Entity,
    pub projectiles: Entity,
}

impl Configure for GameRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for GameRoot {
    fn from_world(world: &mut World) -> Self {
        let players = world
            .spawn((Name::new("Players"), SpatialBundle::default()))
            .id();
        let enemies = world
            .spawn((Name::new("Enemies"), SpatialBundle::default()))
            .id();
        let projectiles = world
            .spawn((Name::new("Projectiles"), SpatialBundle::default()))
            .id();

        Self {
            players,
            enemies,
            projectiles,
        }
    }
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Enemy,
    Projectile,
}
