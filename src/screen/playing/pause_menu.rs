use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::screen::playing::PlayingMenu;
use crate::screen::Screen;
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
    entity.add(widget::blocking_overlay).insert((
        Name::new("PauseOverlay"),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        StateScope::<PlayingMenu>::default(),
    ));
}

fn pause_menu(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("PauseMenu"),
            NodeBundle {
                style: Style::ABS_COLUMN_MID,
                z_index: ZIndex::Global(5000),
                ..default()
            },
            StateScope::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children.spawn_with(header);
            children.spawn_with(button_container);
        });
}

const HEADER: &str = "Paused :)";

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
            margin: UiRect::new(Val::ZERO, Val::ZERO, Vw(3.5), Vw(0.5)),
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
            children.spawn_with(continue_button);
            children.spawn_with(restart_button);
            children.spawn_with(quit_to_title_button);
        });
}

fn continue_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Continue"))
        .insert(On::<Pointer<Click>>::run(PlayingMenu::disable));
}

fn restart_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Restart"))
        // TODO: Fade out?
        .insert(On::<Pointer<Click>>::run(Screen::refresh));
}

fn quit_to_title_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Quit to title"))
        .insert(On::<Pointer<Click>>::run(Screen::Title.enter()));
}
