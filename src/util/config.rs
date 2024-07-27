use std::any::type_name;

use bevy::core::FrameCount;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use iyes_progress::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::util::prelude::*;

pub trait Config: Asset + Serialize + for<'de> Deserialize<'de> {
    const PATH: &'static str;
    const EXTENSION: &'static str;

    fn on_load(&mut self, world: &mut World) {
        let _ = world;
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        let _ = asset_server;
        true
    }

    fn progress(
        asset_server: Res<AssetServer>,
        config_handle: Res<ConfigHandle<Self>>,
        config: Res<Assets<Self>>,
    ) -> Progress {
        config
            .get(&config_handle.0)
            .map(|x| x.is_ready(&asset_server))
            .unwrap_or_default()
            .into()
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ConfigHandle<C: Config>(pub Handle<C>);

impl<C: Config> Configure for ConfigHandle<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_plugins(RonAssetPlugin::<C>::new(&[C::EXTENSION]));
        app.add_systems(Startup, load_config::<C>);
        app.add_systems(
            PreUpdate,
            apply_config::<C>.run_if(on_event::<AssetEvent<C>>()),
        );
    }
}

fn load_config<C: Config>(world: &mut World) {
    let handle = world.resource_mut::<AssetServer>().load(C::PATH);
    world.insert_resource(ConfigHandle::<C>(handle));
}

fn apply_config<C: Config>(world: &mut World, mut reader: Local<ManualEventReader<AssetEvent<C>>>) {
    if !reader
        .read(world.resource::<Events<AssetEvent<_>>>())
        .any(|event| event.is_loaded_with_dependencies(&world.resource::<ConfigHandle<C>>().0))
    {
        return;
    }

    info!(
        "[Frame {}] Applying config: {}",
        world.resource::<FrameCount>().0,
        type_name::<C>()
    );
    world.resource_scope(|world, mut config: Mut<Assets<C>>| {
        let config = r!(config.get_mut(&world.resource::<ConfigHandle<C>>().0));
        config.on_load(world);
    });
}

#[derive(SystemParam)]
pub struct ConfigRef<'w, C: Config> {
    handle: Option<Res<'w, ConfigHandle<C>>>,
    assets: Res<'w, Assets<C>>,
}

impl<C: Config> ConfigRef<'_, C> {
    pub fn get(&self) -> Option<&C> {
        self.handle.as_ref().and_then(|x| self.assets.get(&x.0))
    }
}
