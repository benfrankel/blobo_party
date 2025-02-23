mod intro;
mod loading;
pub mod playing;
mod splash;
mod title;

use bevy::ecs::schedule::SystemConfigs;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::animation::transition::FadeIn;
use crate::animation::transition::FadeOut;
use crate::core::camera::CameraRoot;
use crate::core::window::WindowReady;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure::<Screen>();

    app.add_plugins((
        splash::plugin,
        title::plugin,
        intro::plugin,
        loading::plugin,
        playing::plugin,
    ));
}

#[derive(State, Copy, Clone, Eq, PartialEq, Hash, Debug, Reflect, Default)]
#[state(after(WindowReady), react, bevy_state, log_flush)]
#[reflect(Resource)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Intro,
    Loading,
    Playing,
}

impl Configure for Screen {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_plugins(ProgressPlugin::<BevyState<Self>>::new());
        app.add_systems(
            StateFlush,
            (
                WindowReady.on_enter(Screen::enable_default),
                Screen::ANY.on_exit(reset_camera),
            ),
        );
    }
}

fn reset_camera(camera_root: Res<CameraRoot>, mut camera_query: Query<&mut Transform>) {
    let mut transform = r!(camera_query.get_mut(camera_root.primary));
    transform.translation = Vec2::ZERO.extend(transform.translation.z);
}

const FADE_IN_SECS: f32 = 0.5;

fn fade_in(mut entity: EntityWorldMut) {
    entity.queue(widget::overlay).insert((
        Name::new("ScreenFadeIn"),
        ThemeColor::Body.target::<BackgroundColor>(),
        FadeIn::new(FADE_IN_SECS),
    ));
}

const FADE_OUT_SECS: f32 = 0.2;

fn fade_out(to_screen: Screen) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.queue(widget::blocking_overlay).insert((
            Name::new("ScreenFadeOut"),
            ThemeColor::Body.target::<BackgroundColor>(),
            FadeOut::new(FADE_OUT_SECS, to_screen),
        ));
    }
}

// TODO: This won't work on screen re-entry with pyri_state.
//       Rewrite this abstraction.
pub fn wait(duration: f32) -> SystemConfigs {
    (move |time: Res<Time>, mut start: Local<f32>| -> Progress {
        let elapsed = time.elapsed_secs();
        if *start == 0.0 {
            *start = elapsed;
        }
        let done = elapsed - *start >= duration;

        done.into()
    })
    .track_progress::<BevyState<Screen>>()
}
