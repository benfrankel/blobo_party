use std::time::Duration;

use avian2d::prelude::*;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy_tweening::*;

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

const LIFETIME_SECS: f32 = 0.5;

fn smoke(movement: Vec2) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        let parent = entity.world().resource::<GameRoot>().vfx;
        let vertical = movement.x.abs() / movement.length() < 0.7;
        let (texture, flip_x, flip_y) = if vertical {
            (
                entity
                    .world()
                    .resource::<PlayingAssets>()
                    .vertical_smoke
                    .clone(),
                false,
                movement.y < 0.0,
            )
        } else {
            (
                entity
                    .world()
                    .resource::<PlayingAssets>()
                    .horizontal_smoke
                    .clone(),
                movement.x < 0.0,
                false,
            )
        };

        entity
            .insert((
                Name::new("SmokeVfx"),
                SpriteBundle {
                    sprite: Sprite {
                        flip_x,
                        flip_y,
                        ..default()
                    },
                    texture,
                    ..default()
                },
                Animator::new(Tween::new(
                    EaseMethod::Linear,
                    Duration::from_secs_f32(LIFETIME_SECS),
                    lens::SpriteColorLens {
                        start: Color::WHITE,
                        end: Color::NONE,
                    },
                )),
                RigidBody::Kinematic,
                LinearVelocity(-12.0 * movement),
                DespawnOnTimer(Timer::from_seconds(LIFETIME_SECS, TimerMode::Once)),
            ))
            .set_parent(parent);
    }
}
