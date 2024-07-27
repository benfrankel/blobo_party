use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::extra::entity_scope::StateScope;
use pyri_state::prelude::*;

use crate::core::pause::Pause;
use crate::core::UpdateSet;
use crate::game::actor::level::up::LevelUp;
use crate::game::card::deck::Deck;
use crate::game::card::deck::IsDeckDisplay;
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

    app.configure::<LevelUpMenuAction>();
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
        .add(widget::menu_button("Ready?", Vw(9.0)))
        .insert(On::<Pointer<Click>>::run(PlayingMenu::disable));
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
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.swap(-1);
    }
}

fn card_swap_right(
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.swap(1);
    }
}

fn card_discard(
    deck_display_query: Query<&Selection, With<IsDeckDisplay>>,
    mut deck_query: Query<&mut Deck>,
) {
    for selection in &deck_display_query {
        let mut deck = c!(deck_query.get_mut(selection.0));
        deck.discard();
    }
}
