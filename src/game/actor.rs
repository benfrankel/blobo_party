use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<ActorConfig>>();
}

// TODO: Require all actor assets loaded before continuing to `Screen::Playing`.
#[derive(Asset, Reflect, Serialize, Deserialize)]
struct ActorConfig {
    actors: HashMap<String, Actor>,
    player: String,
}

impl Config for ActorConfig {
    const PATH: &'static str = "config/actor.ron";

    const EXTENSION: &'static str = "actor.ron";

    fn on_load(&mut self, world: &mut World) {
        let mut system_state =
            SystemState::<(Res<AssetServer>, ResMut<Assets<TextureAtlasLayout>>)>::new(world);
        let (asset_server, mut layouts) = system_state.get_mut(world);

        for actor in self.actors.values_mut() {
            actor.texture = asset_server.load(&actor.texture_path);
            actor.texture_atlas_layout = layouts.add(&actor.texture_atlas_grid);
        }
    }
}

#[derive(Reflect, Serialize, Deserialize)]
struct Actor {
    name: String,
    texture_path: String,
    #[serde(skip)]
    texture: Handle<Image>,
    texture_atlas_grid: TextureAtlasGrid,
    #[serde(skip)]
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

fn actor_helper(mut entity: EntityWorldMut, key: Option<String>) {
    let config_handle = entity.world().resource::<ConfigHandle<ActorConfig>>();
    let config = r!(entity
        .world()
        .resource::<Assets<ActorConfig>>()
        .get(&config_handle.0));
    let actor = r!(config.actors.get(key.as_ref().unwrap_or(&config.player)));

    entity.insert((
        Name::new(actor.name.clone()),
        SpriteBundle {
            texture: actor.texture.clone_weak(),
            ..default()
        },
        TextureAtlas {
            layout: actor.texture_atlas_layout.clone_weak(),
            index: 0,
        },
    ));
}

pub fn actor(key: impl Into<String>) -> impl EntityCommand<World> {
    let key = key.into();
    move |entity: EntityWorldMut| {
        actor_helper(entity, Some(key));
    }
}

pub fn player(entity: EntityWorldMut) {
    actor_helper(entity, None);
}
