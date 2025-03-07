use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Title.on_enter(enter_title));
}

fn enter_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(title_screen).set_parent(ui_root.body);
}

fn title_screen(mut entity: EntityWorldMut) {
    entity
        .queue(Node::COLUMN_MID.div())
        .insert(Name::new("TitleScreen"))
        .with_children(|children| {
            children.spawn_with(header);
            children.spawn_with(button_container);
        });
}

const HEADER: &str = "Blobo Party!";

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

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            Node {
                width: Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect::vertical(VMin(9.5)),
                row_gap: Vw(2.5),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(play_button);
            children.spawn_with(quit_button);
        });
}

fn play_button(mut entity: EntityWorldMut) {
    entity.queue(widget::menu_button("Play")).observe(
        |_: Trigger<Pointer<Click>>, mut commands: Commands| {
            commands.spawn_with(fade_out(Screen::Intro));
        },
    );
}

fn quit_button(mut entity: EntityWorldMut) {
    entity.queue(widget::menu_button("Quit"));

    #[cfg(feature = "web")]
    entity.insert(IsDisabled(true));
    #[cfg(not(feature = "web"))]
    entity.observe(|_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<_>| {
        app_exit.send(bevy::app::AppExit::Success);
    });
}
