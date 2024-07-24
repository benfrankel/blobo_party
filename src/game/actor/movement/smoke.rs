use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::core::UpdateSet;
use crate::game::actor::movement::MovementController;
use crate::game::actor::movement::MovementEvent;
use crate::game::cleanup::DespawnOnTimer;
use crate::game::GameRoot;
use crate::screen::playing::PlayingAssets;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_movement_smoke.in_set(UpdateSet::Update));
}

fn spawn_movement_smoke(
    mut commands: Commands,
    mut movement_events: EventReader<MovementEvent>,
    movement_query: Query<(&MovementController, &GlobalTransform)>,
) {
    for event in movement_events.read() {
        let &(MovementEvent::Start(entity) | MovementEvent::Reverse(entity)) = event else {
            continue;
        };
        let (controller, gt) = c!(movement_query.get(entity));
        let mut translation = gt.translation();
        translation.z -= 0.5;

        commands
            .spawn_with(smoke(controller.0))
            .insert(Transform::from_translation(translation));
    }
}

fn smoke(movement: Vec2) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        let parent = entity.world().resource::<GameRoot>().vfx;
        // TODO: Vertical smoke.
        let texture = entity
            .world()
            .resource::<PlayingAssets>()
            .horizontal_smoke
            .clone();

        entity
            .insert((
                Name::new("SmokeVfx"),
                SpriteBundle {
                    sprite: Sprite {
                        flip_x: movement.x < 0.0,
                        ..default()
                    },
                    texture,
                    ..default()
                },
                RigidBody::Kinematic,
                LinearVelocity(-12.0 * movement),
                DespawnOnTimer(Timer::from_seconds(0.2, TimerMode::Once)),
            ))
            .set_parent(parent);
    }
}
