use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::game::actor::health::HealthConfig;
use crate::game::actor::level::LevelConfig;
use crate::game::actor::ActorConfig;
use crate::game::audio::AudioConfig;
use crate::game::card::CardConfig;
use crate::game::combat::projectile::ProjectileConfig;
use crate::game::wave::WaveConfig;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Intro.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Intro.bevy()));
    app.add_systems(StateFlush, Screen::Intro.on_enter(enter_intro));
    app.add_systems(
        Update,
        // TODO: This is kinda silly. Find a better way later.
        Screen::Intro.on_update((
            ActorConfig::progress.track_progress(),
            CardConfig::progress.track_progress(),
            HealthConfig::progress.track_progress(),
            LevelConfig::progress.track_progress(),
            AudioConfig::progress.track_progress(),
            ProjectileConfig::progress.track_progress(),
            WaveConfig::progress.track_progress(),
        )),
    );
}

const HEADER: &str = "How to play:";

fn enter_intro(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(intro_screen).set_parent(ui_root.body);
}

fn intro_screen(mut entity: EntityWorldMut) {
    entity
        .add(Style::COLUMN_MID.div())
        .insert(Name::new("IntroScreen"))
        .with_children(|children| {
            children.spawn_with(header);
            children.spawn_with(body);
            children.spawn_with(button_container);
        });
}

fn header(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("Header"),
        TextBundle::from_section(
            HEADER,
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

fn body(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("Body"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Vw(1.4),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            for (i, text) in [
                "Cards are played to the rhythm,",
                "using your mouse to aim.",
                "Show off your dance moves with 'em,",
                "reach [b]Level 10[r] for fame!",
            ]
            .into_iter()
            .enumerate()
            {
                children.spawn((
                    Name::new(format!("Span{}", i)),
                    TextBundle::from_sections(parse_rich(text)),
                    DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                    ThemeColorForText(vec![
                        ThemeColor::BodyText,
                        ThemeColor::Indicator,
                        ThemeColor::BodyText,
                    ]),
                ));
            }
        });
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
        });
}

fn play_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Let's dance!"))
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
