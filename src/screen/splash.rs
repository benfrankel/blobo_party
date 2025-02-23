use bevy::asset::embedded_asset;
use bevy::core::FrameCount;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::FADE_IN_SECS;
use crate::screen::Screen;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::wait;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "splash/splash.png");

    app.add_systems(StateFlush, Screen::Splash.on_enter(enter_splash));
    app.add_systems(
        Update,
        Screen::Splash.on_update((
            wait(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
            // TODO: System ordering so this runs after all the track progress systems.
            update_splash,
        )),
    );
}

const SPLASH_SCREEN_MIN_SECS: f32 = 0.8;

fn enter_splash(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(splash_screen).set_parent(ui_root.body);
}

fn splash_screen(mut entity: EntityWorldMut) {
    entity
        .queue(Node::COLUMN_CENTER.div())
        .insert(Name::new("SplashScreen"))
        .with_children(|children| {
            children.spawn_with(splash_image);
        });
}

fn splash_image(mut entity: EntityWorldMut) {
    let asset_server = entity.world().resource::<AssetServer>();

    entity.insert((
        Name::new("SplashImage"),
        ImageNode::from(asset_server.load_with_settings(
            "embedded://blobo_party/screen/splash/splash.png",
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::linear();
            },
        )),
        Node {
            margin: UiRect::all(Auto),
            width: Percent(70.0),
            ..default()
        },
        ThemeColor::BodyText.target::<ImageNode>(),
    ));
}

fn update_splash(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready.
    if done == total {
        commands.spawn_with(fade_out(Screen::Title));
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
