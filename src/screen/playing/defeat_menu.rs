use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::extra::entity_scope::StateScope;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::player::IsPlayer;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::Defeat.on_edge(Pause::disable, (Pause::enable_default, open_defeat_menu)),
    );

    app.add_systems(
        Update,
        PlayingMenu::Defeat
            .enter()
            .in_set(UpdateSet::SyncLate)
            .run_if(detect_defeat),
    );
}

fn open_defeat_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(defeat_overlay).set_parent(ui_root.body);
    commands.spawn_with(defeat_menu).set_parent(ui_root.body);
}

pub fn detect_defeat(player_query: Query<Entity, With<IsPlayer>>) -> bool {
    player_query.is_empty()
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
        .insert((
            Name::new("DefeatMenu"),
            NodeBundle {
                style: Style::ABS_COLUMN_MID,
                z_index: ZIndex::Global(2),
                ..default()
            },
            StateScope::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children.spawn_with(header);
            children.spawn_with(button_container);
        });
}

const HEADER: &str = "Defeat :(";

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

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::top(VMin(6.0)),
                    row_gap: Vw(2.5),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(restart_button);
            children.spawn_with(quit_to_title_button);
        });
}

fn restart_button(mut entity: EntityWorldMut) {
    entity.add(widget::menu_button("Restart")).insert((
        On::<Pointer<Click>>::run(
            |mut commands: Commands, audio: Res<Audio>, assets: Res<PlayingAssets>| {
                audio.play(assets.sfx_restart.clone()).with_volume(0.7);
                commands.spawn_with(fade_out(Screen::Playing));
            },
        ),
        Style {
            height: Vw(9.0),
            width: Vw(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}

fn quit_to_title_button(mut entity: EntityWorldMut) {
    entity.add(widget::menu_button("Quit to title")).insert((
        On::<Pointer<Click>>::run(Screen::Title.enter()),
        Style {
            height: Vw(9.0),
            width: Vw(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}
