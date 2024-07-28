//! Game mechanics and content

pub mod actor;
pub mod audio;
pub mod card;
pub mod cleanup;
pub mod combat;
pub mod ground;
pub mod spotlight;
pub mod sprite;
pub mod stats;
pub mod wave;

use std::borrow::Cow;

use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<GameRoot>();

    app.add_plugins((
        actor::plugin,
        audio::plugin,
        card::plugin,
        cleanup::plugin,
        combat::plugin,
        ground::plugin,
        spotlight::plugin,
        sprite::plugin,
        stats::plugin,
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
    pub background: Entity,
}

impl Configure for GameRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Screen::ANY.on_exit(clear_game_root));
    }
}

impl FromWorld for GameRoot {
    fn from_world(world: &mut World) -> Self {
        let players = world.spawn_with(root("Players")).id();
        let enemies = world.spawn_with(root("Enemies")).id();
        let projectiles = world.spawn_with(root("Projectiles")).id();
        let vfx = world.spawn_with(root("Vfx")).id();
        let background = world.spawn_with(root("Background")).id();

        Self {
            players,
            enemies,
            projectiles,
            vfx,
            background,
        }
    }
}

fn clear_game_root(mut commands: Commands, game_root: Res<GameRoot>) {
    commands.entity(game_root.players).despawn_descendants();
    commands.entity(game_root.enemies).despawn_descendants();
    commands.entity(game_root.projectiles).despawn_descendants();
    commands.entity(game_root.vfx).despawn_descendants();
    commands.entity(game_root.background).despawn_descendants();
}

fn root(name: impl Into<Cow<'static, str>>) -> impl EntityCommand<World> {
    let name = name.into();

    move |mut entity: EntityWorldMut| {
        entity.insert((Name::new(name), SpatialBundle::default()));
    }
}

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player,
    Enemy,
    Projectile,
}
