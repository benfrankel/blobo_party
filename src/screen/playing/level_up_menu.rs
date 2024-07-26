use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::level::up::LevelUp;
use crate::screen::playing::PlayingMenu;
use crate::ui::prelude::*;
use crate::util::prelude::*;

// TODO: Deck actions in deck.rs, but disabled by default. Enable them during this menu.
// TODO: Random card selection to add to deck.
// TODO: Helpful message if the player is at deck capacity.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::LevelUp.on_edge(Pause::disable, (Pause::enable_default, open_level_up_menu)),
    );
    app.add_systems(
        Update,
        PlayingMenu::LevelUp
            .enter()
            .in_set(UpdateSet::SyncLate)
            .run_if(on_event::<LevelUp>()),
    );
}

fn open_level_up_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands
        .spawn_with(level_up_overlay)
        .set_parent(ui_root.body);
    commands.spawn_with(level_up_menu).set_parent(ui_root.body);
}

fn level_up_overlay(mut entity: EntityWorldMut) {
    entity.add(widget::overlay).insert((
        Name::new("LevelUpOverlay"),
        ZIndex::Global(1),
        StateScope::<PlayingMenu>::default(),
    ));
}

fn level_up_menu(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("LevelUpMenu"),
            NodeBundle {
                style: Style::ABS_COLUMN_CENTER,
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

const HEADER: &str = "Level up!";

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
            children.spawn_with(ready_button);
        });
}

fn ready_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button("Ready?"))
        .insert(On::<Pointer<Click>>::run(PlayingMenu::disable));
}
