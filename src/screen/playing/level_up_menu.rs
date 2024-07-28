use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::extra::entity_scope::StateScope;
use pyri_state::prelude::*;
use rand::seq::IteratorRandom as _;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::level::up::LevelUp;
use crate::game::card::card;
use crate::game::card::deck::Deck;
use crate::game::card::deck::IsDeckDisplay;
use crate::game::card::CardConfig;
use crate::screen::playing::victory_menu::detect_victory;
use crate::screen::playing::PlayingAssets;
use crate::screen::playing::PlayingMenu;
use crate::ui::prelude::*;
use crate::util::prelude::*;

// TODO: Helpful message if the player is at deck capacity?
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
            .run_if(on_event::<LevelUp>().and_then(not(detect_victory))),
    );

    app.configure::<(LevelUpMenuAction, ToggleDisplay)>();
}

fn open_level_up_menu(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands
        .spawn_with(level_up_overlay)
        .set_parent(ui_root.body);
    commands.spawn_with(level_up_menu).set_parent(ui_root.body);
}

fn level_up_overlay(mut entity: EntityWorldMut) {
    entity.add(widget::blocking_overlay).insert((
        Name::new("LevelUpOverlay"),
        ZIndex::Global(1),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        StateScope::<PlayingMenu>::default(),
    ));
}

fn level_up_menu(mut entity: EntityWorldMut) {
    entity
        .add(Style::ABS_COLUMN_CENTER.div())
        .insert((
            Name::new("LevelUpMenuContainer"),
            StateScope::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("LevelUpMenu"),
                    NodeBundle {
                        style: Style {
                            height: VMin(63.0),
                            top: Vw(-1.7),
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
                    children.spawn_with(instructions_container);
                    children.spawn_with(card_options_container);
                    children.spawn_with(button_container);
                });
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
        ),
        DynamicFontSize::new(Vw(4.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn instructions_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("InstructionsContainer"),
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    row_gap: Vh(2.3),
                    ..default()
                },
                ..default()
            },
            ToggleDisplay,
        ))
        .with_children(|children| {
            for (i, text) in [
                "You can sort your cards during a level up:",
                "",
                "- [b]Select: [r] A/D or Arrow Keys",
                "- [b]Move:   [r] Shift + A/D or Arrow Keys",
                "- [b]Discard:[r] Delete",
            ]
            .into_iter()
            .enumerate()
            {
                children.spawn((
                    Name::new(format!("InstructionsParagraph{}", i)),
                    TextBundle::from_sections(parse_rich(text)),
                    DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                    ThemeColorForText(vec![ThemeColor::BodyText]),
                ));
            }
        });
}

fn card_options_container(entity: Entity, world: &mut World) {
    let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
    let config = r!(config.get());
    // TODO: Pick cards options from a card pool based on level.
    let card_keys = config
        .card_map
        .keys()
        .choose_multiple(&mut rand::thread_rng(), 3)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();

    world
        .entity_mut(entity)
        .insert((
            Name::new("CardOptionsContainer"),
            NodeBundle {
                style: Style {
                    width: Vw(55.0),
                    top: Vw(-1.5),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            ToggleDisplay,
        ))
        .with_children(|children| {
            for key in card_keys {
                children.spawn_with(card_option(key));
            }
        });
}

fn card_option(key: impl Into<String>) -> impl EntityCommand<World> {
    let key = key.into();

    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("CardOption"),
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn_with(card_button(&key));
                children.spawn_with(card_label(key));
            });
    }
}

fn card_button(key: impl Into<String>) -> impl EntityCommand<World> {
    let key = key.into();

    move |mut entity: EntityWorldMut| {
        entity.add(card(key.clone(), None)).insert((
            Interaction::default(),
            On::<Pointer<Click>>::run(
                move |deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
                      mut deck_query: Query<&mut Deck>,
                      mut toggle_query: Query<&mut Style, With<ToggleDisplay>>| {
                    for selection in &deck_display_query {
                        let mut deck = c!(deck_query.get_mut(selection.0));
                        // TODO: What if deck is at capacity?
                        deck.add(key.clone());
                    }
                    for mut style in &mut toggle_query {
                        style.display = match style.display {
                            Display::None => Display::Flex,
                            _ => Display::None,
                        };
                    }
                },
            ),
        ));
    }
}

fn card_label(key: impl Into<String>) -> impl EntityCommand {
    let key = key.into();

    move |entity: Entity, world: &mut World| {
        let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
        let config = r!(config.get());
        let card = r!(config.card_map.get(&key));
        let top = config.card_height * 1.1;
        let text = card.name.clone();

        world.entity_mut(entity).insert((
            Name::new("CardLabel"),
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top,
                    width: Vw(20.0),
                    ..default()
                },
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: FONT_HANDLE,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..default()
            },
            DynamicFontSize::new(Vw(2.0)).with_step(8.0),
            ThemeColorForText(vec![ThemeColor::BodyText]),
        ));
    }
}

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((Name::new("ButtonContainer"), NodeBundle::default()))
        .with_children(|children| {
            children.spawn_with(skip_button);
            children.spawn_with(dance_button);
        });
}

fn skip_button(mut entity: EntityWorldMut) {
    entity.add(widget::menu_button("Skip")).insert((
        On::<Pointer<Click>>::run(
            move |mut toggle_query: Query<&mut Style, With<ToggleDisplay>>| {
                for mut style in &mut toggle_query {
                    style.display = match style.display {
                        Display::None => Display::Flex,
                        _ => Display::None,
                    };
                }
            },
        ),
        Style {
            height: Vw(8.5),
            width: Vw(25.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ToggleDisplay,
    ));
}

fn dance_button(mut entity: EntityWorldMut) {
    entity.add(widget::menu_button("Dance~")).insert((
        On::<Pointer<Click>>::run(PlayingMenu::disable),
        Style {
            display: Display::None,
            height: Vw(8.5),
            width: Vw(25.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ToggleDisplay,
    ));
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum LevelUpMenuAction {
    SelectLeft,
    SelectRight,
    SwapLeft,
    SwapRight,
    Discard,
}

impl Configure for LevelUpMenuAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::SelectLeft, GamepadButtonType::DPadLeft)
                .insert(Self::SelectLeft, GamepadButtonType::LeftTrigger)
                .insert(Self::SelectLeft, KeyCode::KeyA)
                .insert(Self::SelectLeft, KeyCode::ArrowLeft)
                .insert(Self::SelectRight, GamepadButtonType::DPadRight)
                .insert(Self::SelectRight, GamepadButtonType::RightTrigger)
                .insert(Self::SelectRight, KeyCode::KeyD)
                .insert(Self::SelectRight, KeyCode::ArrowRight)
                .insert(Self::SwapLeft, GamepadButtonType::LeftTrigger2)
                .insert_modified(Self::SwapLeft, Modifier::Shift, KeyCode::KeyA)
                .insert_modified(Self::SwapLeft, Modifier::Shift, KeyCode::ArrowLeft)
                .insert(Self::SwapRight, GamepadButtonType::RightTrigger2)
                .insert_modified(Self::SwapRight, Modifier::Shift, KeyCode::KeyD)
                .insert_modified(Self::SwapRight, Modifier::Shift, KeyCode::ArrowRight)
                .insert(Self::Discard, GamepadButtonType::West)
                .insert(Self::Discard, KeyCode::Backspace)
                .insert(Self::Discard, KeyCode::Delete)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        // TODO: It'd be better to disable the action outside of `PlayingMenu::LevelUp`, but
        //       action disabling is buggy in LWIM 0.14. The fix is merged but not yet released.
        app.add_systems(
            Update,
            PlayingMenu::LevelUp.on_update((
                card_select_left
                    .in_set(UpdateSet::RecordInput)
                    .run_if(action_just_pressed(Self::SelectLeft)),
                card_select_right
                    .in_set(UpdateSet::RecordInput)
                    .run_if(action_just_pressed(Self::SelectRight)),
                card_swap_left
                    .in_set(UpdateSet::RecordInput)
                    .run_if(action_just_pressed(Self::SwapLeft)),
                card_swap_right
                    .in_set(UpdateSet::RecordInput)
                    .run_if(action_just_pressed(Self::SwapRight)),
                card_discard
                    .in_set(UpdateSet::RecordInput)
                    .run_if(action_just_pressed(Self::Discard)),
            )),
        );
    }
}

fn card_select_left(
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.advance(-1);
    }
}

fn card_select_right(
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.advance(1);
    }
}

fn card_swap_left(
    audio: Res<Audio>,
    assets: Res<PlayingAssets>,
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    audio.play(assets.sfx_ui_hover.clone());
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.swap(-1);
    }
}

fn card_swap_right(
    audio: Res<Audio>,
    assets: Res<PlayingAssets>,
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    audio.play(assets.sfx_ui_hover.clone());
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.swap(1);
    }
}

fn card_discard(
    audio: Res<Audio>,
    assets: Res<PlayingAssets>,
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    audio.play(assets.sfx_ui_click.clone());
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.discard();
    }
}

/// A marker component for entities that should toggle between
/// `Display::None` and `Display::Flexbox` to swap sub-menus.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ToggleDisplay;

impl Configure for ToggleDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
