use std::marker::PhantomData;

use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::pause::Pause;
use crate::core::PostTransformSet;
use crate::core::UpdateSet;
use crate::game::audio::music::on_beat;
use crate::game::combat::hit::OnHit;
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
        app.observe(apply_despawn_on_hit);
    }
}

fn apply_despawn_on_hit(
    trigger: Trigger<OnHit>,
    mut despawn: ResMut<LateDespawn>,
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
    mut despawn: ResMut<LateDespawn>,
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
            apply_despawn_on_beat
                .in_set(UpdateSet::Update)
                .run_if(on_beat(1)),
        );
    }
}

fn apply_despawn_on_beat(
    mut despawn: ResMut<LateDespawn>,
    mut despawn_query: Query<(Entity, &mut DespawnOnBeat)>,
) {
    for (entity, mut beat) in &mut despawn_query {
        if beat.0 > 0 {
            beat.0 -= 1;
        } else {
            despawn.recursive(entity);
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnOnTimer(pub Timer);

impl Configure for DespawnOnTimer {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            StateFlush,
            Pause.on_edge(unpause_despawn_on_timer, pause_despawn_on_timer),
        );
        app.add_systems(
            Update,
            (
                tick_despawn_on_timer.in_set(UpdateSet::TickTimers),
                apply_despawn_on_timer.in_set(UpdateSet::Update),
            ),
        );
    }
}

fn unpause_despawn_on_timer(mut timer_query: Query<&mut DespawnOnTimer>) {
    for mut timer in &mut timer_query {
        timer.0.unpause();
    }
}

fn pause_despawn_on_timer(mut timer_query: Query<&mut DespawnOnTimer>) {
    for mut timer in &mut timer_query {
        timer.0.pause();
    }
}

fn tick_despawn_on_timer(time: Res<Time>, mut timer_query: Query<&mut DespawnOnTimer>) {
    for mut timer in &mut timer_query {
        timer.0.tick(time.delta());
    }
}

fn apply_despawn_on_timer(
    mut despawn: ResMut<LateDespawn>,
    timer_query: Query<(Entity, &DespawnOnTimer)>,
) {
    for (entity, timer) in &timer_query {
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

impl<C: Component + TypePath> Configure for RemoveOnTimer<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            StateFlush,
            Pause.on_edge(unpause_remove_on_timer::<C>, pause_remove_on_timer::<C>),
        );
        app.add_systems(
            Update,
            (
                tick_remove_on_timer::<C>.in_set(UpdateSet::TickTimers),
                apply_remove_on_timer::<C>.in_set(UpdateSet::SyncLate),
            ),
        );
    }
}

#[allow(dead_code)]
impl<C: Component + TypePath> RemoveOnTimer<C> {
    pub fn new(timer: Timer) -> Self {
        Self {
            timer,
            phantom: PhantomData,
        }
    }

    pub fn bundle(component: C, timer: Timer) -> (C, Self) {
        (component, Self::new(timer))
    }
}

fn tick_remove_on_timer<C: Component + TypePath>(
    time: Res<Time>,
    mut timer_query: Query<&mut RemoveOnTimer<C>>,
) {
    for mut timer in &mut timer_query {
        timer.timer.tick(time.delta());
    }
}

fn unpause_remove_on_timer<C: Component + TypePath>(mut timer_query: Query<&mut RemoveOnTimer<C>>) {
    for mut timer in &mut timer_query {
        timer.timer.unpause();
    }
}

fn pause_remove_on_timer<C: Component + TypePath>(mut timer_query: Query<&mut RemoveOnTimer<C>>) {
    for mut timer in &mut timer_query {
        timer.timer.pause();
    }
}

fn apply_remove_on_timer<C: Component + TypePath>(
    mut commands: Commands,
    timer_query: Query<(Entity, &RemoveOnTimer<C>)>,
) {
    for (entity, timer) in &timer_query {
        if timer.timer.finished() {
            commands.entity(entity).remove::<(C, RemoveOnTimer<C>)>();
        }
    }
}

/// Remove a component after a certain number of eighth-beats.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RemoveOnBeat<C: Component + TypePath> {
    pub beat: usize,
    #[reflect(ignore)]
    phantom: PhantomData<C>,
}

impl<C: Component + TypePath> Configure for RemoveOnBeat<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_remove_on_beat::<C>
                .in_set(UpdateSet::SyncLate)
                .run_if(on_beat(1)),
        );
    }
}

impl<C: Component + TypePath> RemoveOnBeat<C> {
    pub fn new(beat: usize) -> Self {
        Self {
            beat,
            phantom: PhantomData,
        }
    }

    pub fn bundle(component: C, beat: usize) -> (C, Self) {
        (component, Self::new(beat))
    }
}

fn apply_remove_on_beat<C: Component + TypePath>(
    mut commands: Commands,
    mut remove_query: Query<(Entity, &mut RemoveOnBeat<C>)>,
) {
    for (entity, mut remove) in &mut remove_query {
        if remove.beat > 0 {
            remove.beat -= 1;
        } else {
            commands.entity(entity).remove::<(C, RemoveOnBeat<C>)>();
        }
    }
}
