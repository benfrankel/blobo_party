use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off);

    app.configure::<(ConfigHandle<CameraConfig>, CameraRoot, AbsoluteScale)>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct CameraConfig {
    scaling_mode: ScalingMode,
}

impl Config for CameraConfig {
    const PATH: &'static str = "config/camera.ron";

    const EXTENSION: &'static str = "camera.ron";

    fn on_load(&mut self, world: &mut World) {
        let mut projection =
            r!(world.get_mut::<OrthographicProjection>(world.resource::<CameraRoot>().primary));
        projection.scaling_mode = self.scaling_mode;
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CameraRoot {
    pub primary: Entity,
}

impl Configure for CameraRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for CameraRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .spawn((
                    Name::new("PrimaryCamera"),
                    Camera2dBundle {
                        projection: OrthographicProjection {
                            near: -1000.0,
                            ..default()
                        },
                        tonemapping: Tonemapping::None,
                        ..default()
                    },
                    IsDefaultUiCamera,
                ))
                .id(),
        }
    }
}

// Camera zoom-independent scale
// (workaround for https://github.com/bevyengine/bevy/issues/1890)
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct AbsoluteScale(pub Vec3);

impl Configure for AbsoluteScale {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_absolute_scale.in_set(UpdateSet::SyncLate));
    }
}

impl Default for AbsoluteScale {
    fn default() -> Self {
        Self(Vec3::ONE)
    }
}

fn apply_absolute_scale(
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&OrthographicProjection, &Camera)>,
    mut scale_query: Query<(&mut Transform, &AbsoluteScale)>,
) {
    let (camera_proj, camera) = r!(camera_query.get(camera_root.primary));
    let viewport_size = r!(camera.logical_viewport_size());
    let units_per_pixel = camera_proj.area.width() / viewport_size.x;
    let camera_scale_inverse = Vec2::splat(units_per_pixel).extend(1.0);

    for (mut transform, scale) in &mut scale_query {
        transform.scale = camera_scale_inverse * scale.0;
    }
}
