use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use pyri_state::extra::entity_scope::StateScope;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::game::stats::Stats;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        PlayingMenu::Victory.on_edge(Pause::disable, (Pause::enable_default, open_victory_menu)),
    );

    app.configure::<EndlessMode>();
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct EndlessMode(pub bool);

impl Configure for EndlessMode {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Screen::Playing.on_enter(reset_endless_mode));
    }
}

fn reset_endless_mode(mut endless_mode: ResMut<EndlessMode>) {
    endless_mode.0 = false;
}

fn open_victory_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands
        .spawn_with(victory_overlay)
        .set_parent(ui_root.body);
    commands.spawn_with(victory_menu).set_parent(ui_root.body);
}

fn victory_overlay(mut entity: EntityWorldMut) {
    entity.add(widget::blocking_overlay).insert((
        Name::new("VictoryOverlay"),
        ZIndex::Global(1),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        StateScope::<PlayingMenu>::default(),
    ));
}

fn victory_menu(entity: Entity, world: &mut World) {
    let stats = *world.resource::<Stats>();

    world
        .entity_mut(entity)
        .add(Style::ABS_COLUMN_CENTER.div())
        .insert((
            Name::new("VictoryMenuContainer"),
            StateScope::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("VictoryMenu"),
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
                    children.spawn_with(stats);
                    children.spawn_with(button_container);
                });
        });
}

const HEADER: &str = "Life of the party! :)";

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
                    column_gap: Vw(3.8),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(afterparty_button);
            children.spawn_with(restart_button);
            children.spawn_with(quit_button);
        });
}

fn afterparty_button(mut entity: EntityWorldMut) {
    entity
        .add(widget::menu_button_with_font_size("Afterparty", Vw(3.5)))
        .insert((
            On::<Pointer<Click>>::run(
                |mut endless_mode: ResMut<EndlessMode>, mut playing_menu: NextMut<PlayingMenu>| {
                    endless_mode.0 = true;
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
