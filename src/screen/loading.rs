use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::game::actor::facing::FacingAssets;
use crate::game::actor::health::HealthBarConfig;
use crate::game::actor::ActorConfig;
use crate::game::combat::projectile::ProjectileConfig;
use crate::game::level::LevelConfig;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy())
            .load_collection::<PlayingAssets>()
            .load_collection::<FacingAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Loading.bevy()));
    app.add_systems(
        Update,
        Screen::Loading.on_update((
            ActorConfig::progress.track_progress(),
            HealthBarConfig::progress.track_progress(),
            LevelConfig::progress.track_progress(),
            ProjectileConfig::progress.track_progress(),
        )),
    );
    app.add_systems(
        StateFlush,
        Screen::Loading.on_edge(exit_loading, enter_loading),
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
            Screen::Loading.on_update(update_loading_bar.after(TrackedProgressSet)),
        );
    }
}

fn update_loading_bar(
    mut commands: Commands,
    progress: Res<ProgressCounter>,
    frame: Res<FrameCount>,
    mut loading_bar_query: Query<&mut Style, With<IsLoadingBarFill>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready
    if done == total {
        commands.spawn_with(fade_out(Screen::Playing));
    }

    // Update loading bar
    for mut style in &mut loading_bar_query {
        style.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}

fn enter_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(loading_screen).set_parent(ui_root.body);
}

fn exit_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn loading_screen(mut entity: EntityWorldMut) {
    entity
        .add(widget::column_center)
        .insert(Name::new("LoadingScreen"))
        .with_children(|children| {
            children.spawn_with(loading_text);
            children.spawn_with(loading_bar);
        });
}

fn loading_text(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("LoadingText"),
        TextBundle {
            style: Style {
                margin: UiRect::all(Percent(1.0)),
                ..default()
            },
            text: Text::from_section(
                "Loading...",
                TextStyle {
                    font: THICK_FONT_HANDLE,
                    ..default()
                },
            ),
            ..default()
        },
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn loading_bar(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("LoadingBar"),
            NodeBundle {
                style: Style {
                    width: Percent(60.0),
                    height: Percent(8.0),
                    margin: UiRect::all(VMin(2.0)),
                    padding: UiRect::all(VMin(1.0)),
                    border: UiRect::all(VMin(1.0)),
                    ..default()
                },
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
        NodeBundle {
            style: Style {
                width: Percent(0.0),
                height: Percent(100.0),
                ..default()
            },
            ..default()
        },
        ThemeColor::Primary.target::<BackgroundColor>(),
        IsLoadingBarFill,
    ));
}
