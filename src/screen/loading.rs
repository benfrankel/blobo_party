use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::game::actor::ActorConfig;
use crate::game::actor::health::HealthConfig;
use crate::game::actor::level::LevelConfig;
use crate::game::audio::AudioConfig;
use crate::game::card::CardConfig;
use crate::game::combat::projectile::ProjectileConfig;
use crate::game::wave::WaveConfig;
use crate::screen::Screen;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_systems(StateFlush, Screen::Loading.on_enter(enter_loading));
    app.add_systems(
        Update,
        // TODO: This is kinda silly. Find a better way later.
        Screen::Loading.on_update((
            ActorConfig::progress.track_progress::<BevyState<Screen>>(),
            CardConfig::progress.track_progress::<BevyState<Screen>>(),
            HealthConfig::progress.track_progress::<BevyState<Screen>>(),
            LevelConfig::progress.track_progress::<BevyState<Screen>>(),
            AudioConfig::progress.track_progress::<BevyState<Screen>>(),
            ProjectileConfig::progress.track_progress::<BevyState<Screen>>(),
            WaveConfig::progress.track_progress::<BevyState<Screen>>(),
        )),
    );

    app.configure::<IsLoadingBarFill>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsLoadingBarFill;

impl Configure for IsLoadingBarFill {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Screen::Loading.on_update(
                // TODO: System ordering so this runs after all the track progress systems.
                update_loading_bar,
            ),
        );
    }
}

fn update_loading_bar(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut loading_bar_query: Query<&mut Node, With<IsLoadingBarFill>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready
    if done == total {
        commands.spawn_with(fade_out(Screen::Playing));
    }

    // Update loading bar
    for mut node in &mut loading_bar_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}

fn enter_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(loading_screen).set_parent(ui_root.body);
}

fn loading_screen(mut entity: EntityWorldMut) {
    entity
        .queue(Node::COLUMN_CENTER.div())
        .insert(Name::new("LoadingScreen"))
        .with_children(|children| {
            children.spawn_with(loading_text);
            children.spawn_with(loading_bar);
        });
}

fn loading_text(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("LoadingText"),
        Text::new("Loading..."),
        TextFont::from_font(THICK_FONT_HANDLE),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColor::BodyText.target::<TextColor>(),
        Node {
            margin: UiRect::all(Percent(1.0)),
            ..default()
        },
    ));
}

fn loading_bar(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("LoadingBar"),
            Node {
                width: Percent(60.0),
                height: Percent(8.0),
                margin: UiRect::all(VMin(2.0)),
                padding: UiRect::all(VMin(1.0)),
                border: UiRect::all(VMin(1.0)),
                ..default()
            },
            ThemeColor::BodyText.target::<BorderColor>(),
        ))
        .with_children(|children| {
            children.spawn_with(loading_bar_fill);
        });
}

fn loading_bar_fill(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("LoadingBarFill"),
        Node {
            width: Percent(0.0),
            height: Percent(100.0),
            ..default()
        },
        ThemeColor::Primary.target::<BackgroundColor>(),
        IsLoadingBarFill,
    ));
}
