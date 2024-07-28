use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::extra::entity_scope::StateScope;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::game::actor::health::Health;
use crate::game::actor::player::IsPlayer;
use crate::game::combat::death::IsDead;
use crate::game::combat::death::OnDeath;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(detect_defeat);

    app.add_systems(
        StateFlush,
        PlayingMenu::Defeat.on_edge(Pause::disable, (Pause::enable_default, open_defeat_menu)),
    );
}

fn detect_defeat(
    trigger: Trigger<OnDeath>,
    player_query: Query<(), With<IsPlayer>>,
    mut playing_menu: NextMut<PlayingMenu>,
) {
    let entity = r!(trigger.get_entity());
    if !player_query.contains(entity) {
        return;
    }

    playing_menu.enter(PlayingMenu::Defeat);
}

fn open_defeat_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(defeat_overlay).set_parent(ui_root.body);
    commands.spawn_with(defeat_menu).set_parent(ui_root.body);
}

fn defeat_overlay(mut entity: EntityWorldMut) {
    entity.add(widget::blocking_overlay).insert((
        Name::new("DefeatOverlay"),
        ZIndex::Global(1),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        StateScope::<PlayingMenu>::default(),
    ));
}

fn defeat_menu(mut entity: EntityWorldMut) {
    entity
        .add(Style::ABS_COLUMN_CENTER.div())
        .insert((
            Name::new("DefeatMenuContainer"),
            StateScope::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("DefeatMenu"),
                    NodeBundle {
                        style: Style {
                            height: VMin(75.0),
                            top: Vw(-5.2),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        z_index: ZIndex::Global(2),
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn_with(header);
                    children.spawn_with(body);
                    children.spawn_with(button_container);
                });
        });
}

const HEADER: &str = "Party over :(";

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
            margin: UiRect::top(Vw(4.5)),
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
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::auto(2),
                    row_gap: Vw(1.2),
                    column_gap: Vw(2.5),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            // TODO: Real stats.
            for (i, text) in [
                "[b]125",
                "seconds partied",
                "[b]23",
                "blobos impressed",
                "[b]125",
                "dances performed",
                "[b]241",
                "notes played",
                "[b]45",
                "rests taken",
            ]
            .into_iter()
            .enumerate()
            {
                children.spawn((
                    Name::new("BodySpan"),
                    TextBundle::from_sections(parse_rich(text)).with_style(Style {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    }),
                    DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                    ThemeColorForText(vec![if i % 2 == 0 {
                        ThemeColor::Indicator
                    } else {
                        ThemeColor::BodyText
                    }]),
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
                    align_items: AlignItems::Center,
                    column_gap: Vw(3.8),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(dance_on_button);
            children.spawn_with(restart_button);
            children.spawn_with(quit_button);
        });
}

fn dance_on_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button_with_font_size("Dance on", Vw(3.5)))
        .insert((
            On::<Pointer<Click>>::run(
                |mut commands: Commands,
                 mut player_query: Query<(Entity, &mut Health), (With<IsPlayer>, With<IsDead>)>,
                 audio: Res<Audio>,
                 assets: Res<PlayingAssets>,
                 mut playing_menu: NextMut<PlayingMenu>| {
                    for (player, mut health) in &mut player_query {
                        health.current = health.max;
                        commands.entity(player).remove::<IsDead>();
                    }

                    audio.play(assets.sfx_restart.clone()).with_volume(0.7);

                    playing_menu.disable();
                },
            ),
            Style {
                height: Vw(9.0),
                width: Vw(28.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));
}

fn restart_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button_with_font_size("Restart", Vw(3.5)))
        .insert((
            On::<Pointer<Click>>::run(
                |mut commands: Commands, audio: Res<Audio>, assets: Res<PlayingAssets>| {
                    audio.play(assets.sfx_restart.clone()).with_volume(0.7);
                    commands.spawn_with(fade_out(Screen::Playing));
                },
            ),
            Style {
                height: Vw(9.0),
                width: Vw(28.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));
}

fn quit_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button_with_font_size("Quit", Vw(3.5)))
        .insert((
            On::<Pointer<Click>>::run(|mut commands: Commands| {
                commands.spawn_with(fade_out(Screen::Title));
            }),
            Style {
                height: Vw(9.0),
                width: Vw(28.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ));
}
