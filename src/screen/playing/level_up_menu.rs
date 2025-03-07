use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use rand::prelude::*;

use crate::core::UpdateSet;
use crate::core::pause::Pause;
use crate::game::actor::level::Level;
use crate::game::actor::level::up::LevelUp;
use crate::game::card::CardConfig;
use crate::game::card::card;
use crate::game::card::deck::Deck;
use crate::game::card::deck::IsDeckDisplay;
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
            .run_if(on_event::<LevelUp>),
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
    entity.queue(widget::blocking_overlay).insert((
        Name::new("LevelUpOverlay"),
        GlobalZIndex(1),
        ThemeColor::Overlay.target::<BackgroundColor>(),
        DespawnOnExit::<PlayingMenu>::default(),
    ));
}

fn level_up_menu(mut entity: EntityWorldMut) {
    entity
        .queue(Node::ABS_COLUMN_CENTER.div())
        .insert((
            Name::new("LevelUpMenuContainer"),
            DespawnOnExit::<PlayingMenu>::default(),
        ))
        .with_children(|children| {
            children
                .spawn((
                    Name::new("LevelUpMenu"),
                    Node {
                        height: VMin(63.0),
                        top: Vw(-2.1),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    GlobalZIndex(2),
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
        Text::new(HEADER),
        TextFont::from_font(BOLD_FONT_HANDLE),
        DynamicFontSize::new(Vw(4.0)).with_step(8.0),
        ThemeColor::BodyText.target::<TextColor>(),
    ));
}

fn instructions_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("InstructionsContainer"),
            Node {
                display: Display::None,
                flex_direction: FlexDirection::Column,
                row_gap: Vh(2.0),
                ..default()
            },
            ToggleDisplay(Display::Flex),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("FirstLine"),
                Text::new("You can sort your cards during a level up:"),
                TextFont::from_font(FONT_HANDLE),
                DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                ThemeColor::BodyText.target::<TextColor>(),
            ));

            children.spawn((
                Name::new("Blank"),
                Text::default(),
                TextFont::from_font(FONT_HANDLE),
                DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                ThemeColor::BodyText.target::<TextColor>(),
            ));

            children
                .spawn((
                    Name::new("InstructionsGrid"),
                    Node {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::auto(2),
                        row_gap: Vw(0.7),
                        column_gap: Vw(2.5),
                        ..default()
                    },
                ))
                .with_children(|children| {
                    for (i, text) in [
                        "Select",
                        "A/D (or Arrow Keys)",
                        "Move",
                        "Shift + A/D (or Arrow Keys)",
                        "Discard",
                        "Delete",
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        let (font, theme_color, justify_self) = if i % 2 == 0 {
                            (BOLD_FONT_HANDLE, ThemeColor::Indicator, JustifySelf::End)
                        } else {
                            (FONT_HANDLE, ThemeColor::BodyText, JustifySelf::Start)
                        };

                        children.spawn((
                            Name::new(format!("Span{}", i)),
                            Text::new(text),
                            TextFont::from_font(font),
                            theme_color.target::<TextColor>(),
                            DynamicFontSize::new(Vw(3.0)).with_step(8.0),
                            Node {
                                justify_self,
                                ..default()
                            },
                        ));
                    }
                });
        });
}

fn card_options_container(entity: Entity, world: &mut World) {
    let mut system_state = SystemState::<(
        ConfigRef<CardConfig>,
        Query<&Selection, With<IsDeckDisplay>>,
        Query<&Level>,
    )>::new(world);
    let (config, selection_query, level_query) = system_state.get(world);
    let config = r!(config.get());
    let selection = r!(selection_query.get_single());
    let level = r!(level_query.get(selection.0));
    let level = level.current;
    let card_pool = config
        .card_map
        .iter()
        .filter(|(_, card)| card.min_level <= level && level <= card.max_level)
        .collect::<Vec<_>>();
    let card_keys =
        r!(card_pool.choose_multiple_weighted(&mut rand::thread_rng(), 3, |(_, card)| card.weight))
            .map(|(key, _)| (*key).clone())
            .collect::<Vec<_>>();

    world
        .entity_mut(entity)
        .insert((
            Name::new("CardOptionsContainer"),
            Node {
                width: Vw(55.0),
                top: Vw(-1.5),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ToggleDisplay(Display::Flex),
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
                Node {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
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
        entity
            .queue(card(key.clone(), None))
            .insert(Interaction::default())
            .observe(
                move |_: Trigger<Pointer<Click>>,
                      deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
                      mut deck_query: Query<&mut Deck>,
                      mut toggle_query: Query<(&mut Node, &ToggleDisplay)>| {
                    for selection in &deck_display_query {
                        let mut deck = c!(deck_query.get_mut(selection.0));
                        // TODO: What if deck is at capacity?
                        deck.add(key.clone());
                    }
                    for (mut node, display) in &mut toggle_query {
                        node.display = match node.display {
                            Display::None => display.0,
                            _ => Display::None,
                        };
                    }
                },
            );
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
            Text::new(text),
            TextFont::from_font(FONT_HANDLE),
            TextLayout::new_with_justify(JustifyText::Center),
            DynamicFontSize::new(Vw(2.0)).with_step(8.0),
            ThemeColor::BodyText.target::<TextColor>(),
            Node {
                position_type: PositionType::Absolute,
                top,
                width: Vw(20.0),
                ..default()
            },
        ));
    }
}

fn button_container(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("ButtonContainer"),
            Node {
                top: Vh(0.7),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(skip_button);
            children.spawn_with(ready_button);
        });
}

fn skip_button(mut entity: EntityWorldMut) {
    entity
        .queue(widget::menu_button("Skip"))
        .insert((
            Node {
                height: Vw(8.5),
                width: Vw(25.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ToggleDisplay(Display::Flex),
        ))
        .observe(
            move |_: Trigger<Pointer<Click>>,
                  mut toggle_query: Query<(&mut Node, &ToggleDisplay)>| {
                for (mut node, display) in &mut toggle_query {
                    node.display = match node.display {
                        Display::None => display.0,
                        _ => Display::None,
                    };
                }
            },
        );
}

fn ready_button(mut entity: EntityWorldMut) {
    entity
        .queue(widget::menu_button("Ready"))
        .insert((
            Node {
                display: Display::None,
                height: Vw(8.5),
                width: Vw(25.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ToggleDisplay(Display::Flex),
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut menu: NextMut<PlayingMenu>| menu.disable());
}

#[derive(Actionlike, Eq, PartialEq, Hash, Copy, Clone, Reflect, Debug)]
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
                .with(Self::SelectLeft, GamepadButton::DPadLeft)
                .with(Self::SelectLeft, GamepadButton::LeftTrigger)
                .with(Self::SelectLeft, KeyCode::KeyA)
                .with(Self::SelectLeft, KeyCode::ArrowLeft)
                .with(Self::SelectRight, GamepadButton::DPadRight)
                .with(Self::SelectRight, GamepadButton::RightTrigger)
                .with(Self::SelectRight, KeyCode::KeyD)
                .with(Self::SelectRight, KeyCode::ArrowRight)
                .with(Self::SwapLeft, GamepadButton::LeftTrigger2)
                .with(Self::SwapLeft, ModifierKey::Shift.with(KeyCode::KeyA))
                .with(Self::SwapLeft, ModifierKey::Shift.with(KeyCode::ArrowLeft))
                .with(Self::SwapRight, GamepadButton::RightTrigger2)
                .with(Self::SwapRight, ModifierKey::Shift.with(KeyCode::KeyD))
                .with(
                    Self::SwapRight,
                    ModifierKey::Shift.with(KeyCode::ArrowRight),
                )
                .with(Self::Discard, GamepadButton::West)
                .with(Self::Discard, KeyCode::Backspace)
                .with(Self::Discard, KeyCode::Delete),
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
    audio
        .play(assets.sfx_ui_hover.clone())
        .with_playback_rate(rand::thread_rng().gen_range(0.7..1.6));
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
    audio
        .play(assets.sfx_ui_hover.clone())
        .with_playback_rate(rand::thread_rng().gen_range(0.7..1.6));
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
    audio
        .play(assets.sfx_ui_click_fail.clone())
        .with_playback_rate(rand::thread_rng().gen_range(0.8..1.4));
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.discard();
    }
}

/// A marker component for entities that should toggle between
/// `Display::None` and `Display::Flexbox` to swap sub-menus.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ToggleDisplay(Display);

impl Configure for ToggleDisplay {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
