//! Game mechanics and content

pub mod actor;
pub mod card;
pub mod cleanup;
pub mod combat;
pub mod level;
pub mod music;
pub mod spotlight;
pub mod sprite;
pub mod wave;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GameRoot>();

    app.add_plugins((
        actor::plugin,
        card::plugin,
        cleanup::plugin,
        combat::plugin,
        level::plugin,
        music::plugin,
        spotlight::plugin,
        sprite::plugin,
        wave::plugin,
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameRoot {
    pub players: Entity,
    pub enemies: Entity,
    pub projectiles: Entity,
    pub vfx: Entity,
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
        let vfx = world
            .spawn((Name::new("Vfx"), SpatialBundle::default()))
            .id();

        Self {
            players,
            enemies,
            projectiles,
            vfx,
        }
    }
}

impl GameRoot {
    pub fn despawn_descendants(&self, commands: &mut Commands) {
        commands.entity(self.players).despawn_descendants();
        commands.entity(self.enemies).despawn_descendants();
        commands.entity(self.projectiles).despawn_descendants();
        commands.entity(self.vfx).despawn_descendants();
    }
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Enemy,
    Projectile,
}
