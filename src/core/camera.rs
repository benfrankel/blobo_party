use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use pyri_state::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Msaa::Off);

    app.configure::<(
        ConfigHandle<CameraConfig>,
        CameraRoot,
        SmoothFollow,
        AbsoluteScale,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
struct CameraConfig {
    scaling_mode: ScalingMode,
    follow_rate: Vec2,
}

impl Config for CameraConfig {
    const PATH: &'static str = "config/camera.ron";
    const EXTENSION: &'static str = "camera.ron";

    fn on_load(&mut self, world: &mut World) {
        let (mut projection, mut follow) = r!(world
            .query::<(&mut OrthographicProjection, &mut SmoothFollow)>()
            .get_mut(world, world.resource::<CameraRoot>().primary));

        projection.scaling_mode = self.scaling_mode;
        follow.rate = self.follow_rate;
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
        app.add_systems(StateFlush, Screen::ANY.on_exit(clear_camera_root));
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
                    SmoothFollow {
                        target: Entity::PLACEHOLDER,
                        rate: Vec2::splat(100.0),
                    },
                    IsDefaultUiCamera,
                ))
                .id(),
        }
    }
}

fn clear_camera_root(mut commands: Commands, camera_root: Res<CameraRoot>) {
    commands.entity(camera_root.primary).despawn_descendants();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SmoothFollow {
    pub target: Entity,
    pub rate: Vec2,
}

impl Configure for SmoothFollow {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_smooth_follow.run_if(Pause::is_disabled));
    }
}

fn apply_smooth_follow(
    time: Res<Time>,
    mut follow_query: Query<(&mut Transform, &mut GlobalTransform, &SmoothFollow)>,
    target_query: Query<&GlobalTransform, Without<SmoothFollow>>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut gt, follow) in &mut follow_query {
        let Ok(target) = target_query.get(follow.target) else {
            continue;
        };

        let target_pos = target.translation().xy();
        let mut pos = transform.translation.xy();
        pos += (target_pos - pos) * (follow.rate * dt).clamp(Vec2::ZERO, Vec2::ONE);

        transform.translation = pos.extend(transform.translation.z);
        // TODO: This is a bit of a hack because transform propagation is awkward.
        *gt = (*transform).into();
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
