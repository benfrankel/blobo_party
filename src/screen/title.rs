use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::game::actor::health::HealthConfig;
use crate::game::actor::ActorConfig;
use crate::game::card::CardConfig;
use crate::game::combat::projectile::ProjectileConfig;
use crate::game::level::LevelConfig;
use crate::game::music::MusicConfig;
use crate::game::wave::WaveConfig;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Title.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Title.bevy()));
    app.add_systems(StateFlush, Screen::Title.on_edge(exit_title, enter_title));
    app.add_systems(
        Update,
        // TODO: This is kinda silly. Find a better way later.
        Screen::Title.on_update((
            ActorConfig::progress.track_progress(),
            CardConfig::progress.track_progress(),
            HealthConfig::progress.track_progress(),
            LevelConfig::progress.track_progress(),
            MusicConfig::progress.track_progress(),
            ProjectileConfig::progress.track_progress(),
            WaveConfig::progress.track_progress(),
        )),
    );

    app.configure::<TitleScreenAssets>();
}

const TITLE: &str = "Blobo Party!";

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

impl Configure for TitleScreenAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn enter_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(title_screen).set_parent(ui_root.body);
}

fn exit_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn title_screen(mut entity: EntityWorldMut) {
    entity
        .add(widget::column_mid)
        .insert(Name::new("TitleScreen"))
        .with_children(|children| {
            children.spawn_with(title_text);
            children.spawn_with(button_container);
        });
}

fn title_text(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("TitleText"),
        TextBundle::from_section(
            TITLE,
            TextStyle {
                font: BOLD_FONT_HANDLE,
                ..default()
            },
        )
        .with_style(Style {
            margin: UiRect::vertical(Vw(5.0)),
            ..default()
        }),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::vertical(VMin(9.0)),
                    row_gap: Vw(2.5),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(play_button);
            children.spawn_with(quit_button);
        });
}

fn play_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Play"))
        .insert(On::<Pointer<Click>>::run(
            |mut commands: Commands, progress: Res<ProgressCounter>| {
                let Progress { done, total } = progress.progress_complete();
                commands.spawn_with(fade_out(if done >= total {
                    Screen::Playing
                } else {
                    Screen::Loading
                }));
            },
        ));
}

fn quit_button(mut entity: EntityWorldMut) {
    entity.add(widget::menu_button("Quit")).insert((
        #[cfg(feature = "web")]
        IsDisabled(true),
        #[cfg(not(feature = "web"))]
        On::<Pointer<Click>>::run(|mut app_exit: EventWriter<_>| {
            app_exit.send(bevy::app::AppExit::Success);
        }),
    ));
}
