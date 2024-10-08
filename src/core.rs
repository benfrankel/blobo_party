//! Foundational features and cross-cutting concerns.

pub mod asset;
pub mod audio;
pub mod camera;
#[cfg(feature = "dev")]
pub mod debug;
pub mod pause;
pub mod physics;
pub mod state;
pub mod theme;
pub mod window;

use avian2d::prelude::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::ui::UiSystem;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(UpdateSet, PostTransformSet, PostColorSet)>();

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .build()
            // TODO: Doing this instead of `.replace` because `window::plugin` requires `AssetPlugin` to load its config.
            .disable::<AssetPlugin>()
            .add_after::<LogPlugin, _>(asset::plugin)
            .add_after::<LogPlugin, _>(state::plugin)
            .replace::<WindowPlugin>(window::plugin)
            .set(ImagePlugin::default_nearest()),
    );

    // Add other plugins.
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        #[cfg(feature = "dev")]
        debug::plugin,
        pause::plugin,
        physics::plugin,
        theme::plugin,
    ));
}

// TODO: This would fit better in `game.rs`.
/// Game logic system ordering in the [`Update`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UpdateSet {
    /// Tick timers.
    TickTimers,
    /// Synchronize start-of-frame values.
    SyncEarly,
    /// Play cards.
    PlayCards,
    /// Record player and AI input.
    RecordInput,
    /// Step game logic.
    Update,
    /// Detect health and trigger death.
    TriggerDeath,
    /// Detect XP and trigger level up.
    TriggerLevelUp,
    /// Despawn entities.
    Despawn,
    /// Spawn entities.
    Spawn,
    /// Synchronize end-of-frame values.
    SyncLate,
}

impl Configure for UpdateSet {
    fn configure(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Self::TickTimers,
                Self::SyncEarly,
                Self::PlayCards,
                Self::RecordInput,
                Self::Update,
                Self::TriggerDeath,
                Self::TriggerLevelUp,
                Self::Despawn,
                Self::Spawn,
                Self::SyncLate,
            )
                .chain(),
        );
    }
}

// TODO: This would fit better in `animation.rs`.
/// [`Transform`] post-processing system ordering in the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostTransformSet {
    /// Save the base transform as a backup.
    Save,
    /// Blend via transform multiplication (add translation, add rotation, multiply scale).
    Blend,
    /// Apply facing (may multiply translation.x by -1).
    ApplyFacing,
    /// Apply finishing touches to GlobalTransform, like rounding to the nearest pixel.
    Finish,
}

impl Configure for PostTransformSet {
    fn configure(app: &mut App) {
        app.configure_sets(
            PostUpdate,
            (
                (UiSystem::Layout, PhysicsSet::Sync),
                Self::Save,
                Self::Blend,
                Self::ApplyFacing,
                TransformSystem::TransformPropagate,
                Self::Finish,
                // GlobalTransform may be slightly out of sync with Transform at this point...
            )
                .chain(),
        );
    }
}

// TODO: This would fit better in `animation.rs`.
/// [`Color`] post-processing system ordering in the [`PostUpdate`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PostColorSet {
    /// Save the base color as a backup.
    Save,
    /// Blend via color multiplication (multiply RGBA).
    Blend,
}

impl Configure for PostColorSet {
    fn configure(app: &mut App) {
        app.configure_sets(PostUpdate, (Self::Save, Self::Blend).chain());
    }
}
