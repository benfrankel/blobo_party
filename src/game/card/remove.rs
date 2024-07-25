use std::marker::PhantomData;

use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::music::beat::on_beat;
use crate::util::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RemoveOnTimer<C: Component + TypePath> {
    pub timer: Timer,
    #[reflect(ignore)]
    phantom: PhantomData<C>,
}

impl<C: Component + TypePath> RemoveOnTimer<C> {
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
