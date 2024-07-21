use bevy::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::window::WindowRoot;
use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Facing, FacePlayer, FaceCursor)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Facing(pub Dir2);

impl Configure for Facing {
    fn configure(app: &mut App) {
        app.register_type::<Facing>();
    }
}

impl Default for Facing {
    fn default() -> Self {
        Self(Dir2::EAST)
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FacePlayer;

impl Configure for FacePlayer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, face_player.in_set(UpdateSet::SyncEarly));
    }
}

fn face_player(
    player_query: Query<&GlobalTransform, With<IsPlayer>>,
    mut facing_query: Query<(&mut Facing, &GlobalTransform), With<FacePlayer>>,
) {
    let target_pos = r!(player_query.get_single()).translation().xy();

    for (mut facing, gt) in &mut facing_query {
        let pos = gt.translation().xy();
        facing.0 = c!(Dir2::new(target_pos - pos));
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FaceCursor;

impl Configure for FaceCursor {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, face_cursor.in_set(UpdateSet::SyncEarly));
    }
}

fn face_cursor(
    window_root: Res<WindowRoot>,
    window_query: Query<&Window>,
    camera_root: Res<CameraRoot>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut facing_query: Query<(&mut Facing, &GlobalTransform), With<FaceCursor>>,
) {
    let window = r!(window_query.get(window_root.primary));
    let (camera, camera_gt) = r!(camera_query.get(camera_root.primary));
    let target_pos = r!(window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_gt, cursor)));

    for (mut facing, gt) in &mut facing_query {
        let pos = gt.translation().xy();
        facing.0 = c!(Dir2::new(target_pos - pos));
    }
}
