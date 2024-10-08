use bevy::asset::embedded_asset;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::texture::ImageLoaderSettings;
use bevy::render::texture::ImageSampler;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::Screen;
use crate::screen::FADE_IN_SECS;
use crate::ui::prelude::*;
use crate::util::prelude::*;
use crate::util::time::wait;

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "splash/splash.png");

    app.add_plugins(ProgressPlugin::new(Screen::Splash.bevy()));
    app.add_systems(StateFlush, Screen::Splash.on_enter(enter_splash));
    app.add_systems(
        Update,
        Screen::Splash.on_update((
            wait(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
            update_splash.after(TrackedProgressSet),
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
        .add(Style::COLUMN_CENTER.div())
        .insert(Name::new("SplashScreen"))
        .with_children(|children| {
            children.spawn_with(splash_image);
        });
}

fn splash_image(mut entity: EntityWorldMut) {
    let asset_server = entity.world().resource::<AssetServer>();

    entity.insert((
        Name::new("SplashImage"),
        ImageBundle {
            style: Style {
                margin: UiRect::all(Auto),
                width: Percent(70.0),
                ..default()
            },
            image: UiImage::new(asset_server.load_with_settings(
                "embedded://blobo_party/screen/splash/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            )),
            ..default()
        },
        ThemeColor::BodyText.target::<UiImage>(),
    ));
}

fn update_splash(
    mut commands: Commands,
    progress: Res<ProgressCounter>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready
    if done == total {
        commands.spawn_with(fade_out(Screen::Title));
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
