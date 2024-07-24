use bevy::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::combat::hit::OnHit;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(
        DespawnOnHit,
        DespawnOnDistanceSq,
        DespawnOnBeat,
        DespawnOnTimer,
    )>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnHit;

impl Configure for DespawnOnHit {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.observe(despawn_on_hit);
    }
}

fn despawn_on_hit(
    trigger: Trigger<OnHit>,
    mut despawn: ResMut<DespawnSet>,
    despawn_query: Query<(), With<DespawnOnHit>>,
) {
    let hitbox = trigger.event().0;
    if despawn_query.contains(hitbox) {
        despawn.recursive(hitbox);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnDistanceSq(pub f32);

impl Configure for DespawnOnDistanceSq {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            despawn_on_distance_sq.after(PostTransformSet::Finish),
        );
    }
}

impl DespawnOnDistanceSq {
    pub fn new(distance: f32) -> Self {
        Self(distance * distance)
    }
}

fn despawn_on_distance_sq(
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    mut despawn: ResMut<DespawnSet>,
    despawn_query: Query<(Entity, &GlobalTransform, &DespawnOnDistanceSq)>,
) {
    let camera_gt = r!(camera_query.get(camera_root.primary));
    let camera_pos = camera_gt.translation().xy();

    for (entity, gt, distance) in &despawn_query {
        if gt.translation().xy().distance_squared(camera_pos) >= distance.0 {
            despawn.recursive(entity);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnBeat(pub usize);

impl Configure for DespawnOnBeat {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            despawn_on_beat.in_set(UpdateSet::Update).run_if(on_beat(1)),
        );
    }
}

fn despawn_on_beat(
    mut despawn: ResMut<DespawnSet>,
    mut despawn_query: Query<(Entity, &mut DespawnOnBeat)>,
) {
    for (entity, mut beat) in &mut despawn_query {
        if beat.0 <= 1 {
            despawn.recursive(entity);
        }
        beat.0 -= 1;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnTimer(pub Timer);

impl Configure for DespawnOnTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, despawn_on_timer.in_set(UpdateSet::Update));
    }
}

fn despawn_on_timer(
    time: Res<Time>,
    mut despawn: ResMut<DespawnSet>,
    mut despawn_query: Query<(Entity, &mut DespawnOnTimer)>,
) {
    for (entity, mut timer) in &mut despawn_query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            despawn.recursive(entity);
        }
    }
}
