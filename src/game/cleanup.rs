use std::marker::PhantomData;

use bevy::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::combat::hit::OnHit;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(DespawnOnHit, DespawnRadiusSq, DespawnOnBeat, DespawnOnTimer)>();
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
pub struct DespawnRadiusSq(pub f32);

impl Configure for DespawnRadiusSq {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            PostUpdate,
            apply_despawn_radius_sq.after(PostTransformSet::Finish),
        );
    }
}

impl DespawnRadiusSq {
    pub fn new(distance: f32) -> Self {
        Self(distance * distance)
    }
}

fn apply_despawn_radius_sq(
    camera_root: Res<CameraRoot>,
    camera_query: Query<&GlobalTransform>,
    mut despawn: ResMut<DespawnSet>,
    despawn_query: Query<(Entity, &GlobalTransform, &DespawnRadiusSq)>,
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RemoveOnTimer<C: Component + TypePath> {
    pub timer: Timer,
    #[reflect(ignore)]
    phantom: PhantomData<C>,
}

impl<C: Component + TypePath> RemoveOnTimer<C> {
    #[allow(dead_code)]
    pub fn new(timer: Timer) -> Self {
        RemoveOnTimer {
            timer,
            phantom: PhantomData::<C>,
        }
    }
}

impl<C: Component + TypePath> Configure for RemoveOnTimer<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            remove_on_timer::<C>.in_set(UpdateSet::SyncLate), // TODO: Is this the best choice?
        );
    }
}

fn remove_on_timer<C: Component + TypePath>(
    mut commands: Commands,
    mut components_to_remove: Query<(Entity, &mut RemoveOnTimer<C>)>,
    time: Res<Time>,
) {
    for (entity, mut component) in &mut components_to_remove {
        if component.timer.tick(time.delta()).finished() {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.remove::<C>();
                entity.remove::<RemoveOnTimer<C>>();
            }
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RemoveOnBeat<C: Component + TypePath> {
    pub count: usize,
    #[reflect(ignore)]
    phantom: PhantomData<C>,
}

impl<C: Component + TypePath> RemoveOnBeat<C> {
    pub fn new(count: usize) -> Self {
        RemoveOnBeat {
            count,
            phantom: PhantomData::<C>,
        }
    }
}

impl<C: Component + TypePath> Configure for RemoveOnBeat<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            remove_on_beat::<C>.in_set(UpdateSet::SyncLate) // TODO: Is this the best choice?
.run_if(on_beat(1)),
        );
    }
}

fn remove_on_beat<C: Component + TypePath>(
    mut commands: Commands,
    mut components_to_remove: Query<(Entity, &mut RemoveOnBeat<C>)>,
) {
    for (entity, mut component) in &mut components_to_remove {
        if component.count == 0 {
            if let Some(mut entity) = commands.get_entity(entity) {
                entity.remove::<C>();
                entity.remove::<RemoveOnBeat<C>>();
            }
        }
        component.count = component.count.saturating_sub(1);
    }
}
