use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::screen::Screen;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::Pause.on_edge(Pause::disable, (Pause::enable_default, open_pause_menu)),
    );
}

fn open_pause_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(pause_overlay).set_parent(ui_root.body);
    commands.spawn_with(pause_menu).set_parent(ui_root.body);
}

fn pause_overlay(mut entity: EntityWorldMut) {
    entity.queue(widget::blocking_overlay).insert((
        Name::new("PauseOverlay"),
        GlobalZIndex(1),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        DespawnOnExit::<PlayingMenu>::default(),
    ));
}

fn pause_menu(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("PauseMenu"),
            Node::ABS_COLUMN_MID,
            GlobalZIndex(2),
            DespawnOnExit::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children.spawn_with(header);
            children.spawn_with(button_container);
        });
}

const HEADER: &str = "Paused :|";

fn header(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("Header"),
        Text::new(HEADER),
        TextFont::from_font(BOLD_FONT_HANDLE),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColor::BodyText.target::<TextColor>(),
        Node {
            margin: UiRect::top(Vw(4.5)),
            ..default()
        },
    ));
}

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            Node {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::top(VMin(6.0)),
                row_gap: Vw(2.5),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(continue_button);
            children.spawn_with(restart_button);
            children.spawn_with(quit_to_title_button);
        });
}

fn continue_button(mut entity: EntityWorldMut) {
    entity
        .queue(widget::menu_button("Continue"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .observe(|_: Trigger<Pointer<Click>>, mut menu: NextMut<PlayingMenu>| menu.disable());
}

fn restart_button(mut entity: EntityWorldMut) {
    entity
        .queue(widget::menu_button("Restart"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .observe(
            |_: Trigger<Pointer<Click>>,
             mut commands: Commands,
             audio: Res<Audio>,
             assets: Res<PlayingAssets>| {
                commands.spawn_with(fade_out(Screen::Playing));
                audio.play(assets.sfx_restart.clone()).with_volume(0.7);
            },
        );
}

fn quit_to_title_button(mut entity: EntityWorldMut) {
    entity
        .queue(widget::menu_button("Quit to title"))
        .insert(Node {
            height: Vw(9.0),
            width: Vw(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.spawn_with(fade_out(Screen::Title));
        });
}
