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
        LoadingState::new(Screen::Intro.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_systems(StateFlush, Screen::Intro.on_enter(enter_intro));
    app.add_systems(
        Update,
        // TODO: This is kinda silly. Find a better way later.
        Screen::Intro.on_update((
            ActorConfig::progress.track_progress::<BevyState<Screen>>(),
            CardConfig::progress.track_progress::<BevyState<Screen>>(),
            HealthConfig::progress.track_progress::<BevyState<Screen>>(),
            LevelConfig::progress.track_progress::<BevyState<Screen>>(),
            AudioConfig::progress.track_progress::<BevyState<Screen>>(),
            ProjectileConfig::progress.track_progress::<BevyState<Screen>>(),
            WaveConfig::progress.track_progress::<BevyState<Screen>>(),
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
        .queue(Node::COLUMN_MID.div())
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
        Text::new(HEADER),
        TextFont::from_font(BOLD_FONT_HANDLE),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColor::BodyText.target::<TextColor>(),
        Node {
            margin: UiRect::top(Vw(4.5)).with_bottom(Vw(4.2)),
            ..default()
        },
    ));
}

fn body(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("Body"),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Vw(0.8),
                ..default()
            },
        ))
        .with_children(|children| {
            for (i, text) in [
                "Cards are played to the rhythm,",
                "using your mouse to aim.",
                "Show off your dance moves with 'em,",
                "reach ",
            ]
            .into_iter()
            .enumerate()
            {
                let mut line = children.spawn((
                    Name::new(format!("Line{}", i)),
                    Text::new(text),
                    TextFont::from_font(FONT_HANDLE),
                    DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                    ThemeColor::BodyText.target::<TextColor>(),
                ));
                if i == 3 {
                    line.with_children(|children| {
                        children.spawn((
                            Name::new("Span1"),
                            TextSpan::new("Level 10"),
                            TextFont::from_font(BOLD_FONT_HANDLE),
                            DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                            ThemeColor::Indicator.target::<TextColor>(),
                        ));
                        children.spawn((
                            Name::new("Span2"),
                            TextSpan::new(" for fame!"),
                            TextFont::from_font(FONT_HANDLE),
                            DynamicFontSize::new(Vw(3.5)).with_step(8.0),
                            ThemeColor::BodyText.target::<TextColor>(),
                        ));
                    });
                }
            }
        });
}

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            Node {
                width: Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(VMin(8.5)),
                row_gap: Vw(2.5),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(play_button);
        });
}

fn play_button(mut entity: EntityWorldMut) {
    entity.queue(widget::menu_button("Let's dance!")).observe(
        |_: Trigger<Pointer<Click>>,
         mut commands: Commands,
         progress: Res<ProgressTracker<BevyState<Screen>>>| {
            let Progress { done, total } = progress.get_global_combined_progress();
            commands.spawn_with(fade_out(if done >= total {
                Screen::Playing
            } else {
                Screen::Loading
            }));
        },
    );
}
