use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::animation::backup::Backup;
use crate::core::camera::CameraRoot;
use crate::core::theme::ThemeColor;
use crate::core::window::WindowRoot;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(Facing, FacePlayer, FaceCursor, FacingIndicator)>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Facing(pub Dir2);

impl Configure for Facing {
    fn configure(app: &mut App) {
        app.register_type::<Facing>();
        app.add_systems(Update, apply_facing_to_sprite.in_set(UpdateSet::Update));
    }
}

impl Default for Facing {
    fn default() -> Self {
        Self(Dir2::EAST)
    }
}

fn apply_facing_to_sprite(mut facing_query: Query<(&Facing, &mut Sprite)>) {
    for (facing, mut sprite) in &mut facing_query {
        if facing.0.x != 0.0 {
            sprite.flip_x = facing.0.x < 0.0;
        }
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
    let target_pos = rq!(window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_gt, cursor)));

    for (mut facing, gt) in &mut facing_query {
        let pos = gt.translation().xy();
        facing.0 = c!(Dir2::new(target_pos - pos));
    }
}

/// Reads from the `Facing` component on its parent entity.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct FacingIndicator {
    pub offset: Vec2,
}

impl Configure for FacingIndicator {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            update_facing_indicator.in_set(PostTransformSet::Blend),
        );
    }
}

fn update_facing_indicator(
    facing_query: Query<&Facing>,
    mut facing_indicator_query: Query<(&Parent, &FacingIndicator, &mut Transform)>,
) {
    for (parent, facing_indicator, mut transform) in &mut facing_indicator_query {
        let facing = c!(facing_query.get(parent.get()));
        transform.translation += (facing_indicator.offset * facing.0.as_vec2()).extend(0.0);
        transform.rotate_z(facing.0.to_angle());
    }
}

impl EntityCommand for FacingIndicator {
    fn apply(self, id: Entity, world: &mut World) {
        let texture = world.resource::<PlayingAssets>().mini_arrow.clone();

        world.entity_mut(id).insert((
            Name::new("FacingIndicator"),
            SpriteBundle {
                texture,
                ..default()
            },
            ThemeColor::Indicator.target::<Sprite>(),
            Backup::<Transform>::default(),
            self,
        ));
    }
}
