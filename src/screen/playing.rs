pub mod hud;
pub mod level_up_menu;
pub mod pause_menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use pyri_state::schedule::ResolveStateSet;

use crate::core::pause::Pause;
use crate::game::actor::player::player;
use crate::game::spotlight::spotlight_lamp_spawner;
use crate::game::GameRoot;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::hud::playing_hud;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Playing.on_enter(enter_playing));

    app.configure::<(PlayingAssets, PlayingAction, PlayingMenu)>();

    app.add_plugins((level_up_menu::plugin, pause_menu::plugin));
}

fn enter_playing(mut commands: Commands, game_root: Res<GameRoot>, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);

    // TODO: Character select screen.
    let player = commands.spawn_with(player("pink")).id();

    commands
        .spawn_with(spotlight_lamp_spawner)
        .set_parent(game_root.vfx);

    commands
        .spawn_with(playing_hud(player))
        .set_parent(ui_root.body);
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayingAssets {
    #[asset(path = "image/ui/mini_arrow.png")]
    pub mini_arrow: Handle<Image>,
    #[asset(path = "image/ui/arrow.png")]
    pub arrow: Handle<Image>,
    #[asset(path = "image/ui/simple_border.png")]
    pub simple_border: Handle<Image>,
    #[asset(path = "image/vfx/horizontal_smoke.png")]
    pub horizontal_smoke: Handle<Image>,
    #[asset(path = "image/vfx/vertical_smoke.png")]
    pub vertical_smoke: Handle<Image>,
    #[asset(path = "image/vfx/spotlight.png")]
    pub spotlight: Handle<Image>,
    #[asset(path = "image/vfx/spotlight_lamp.png")]
    pub spotlight_lamp: Handle<Image>,
}

impl Configure for PlayingAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum PlayingAction {
    Restart,
    TogglePause,
    // TODO: These actions should be split out.
    // TODO: Discard action.
    AddCard,
    SelectCardRight,
    SelectCardLeft,
    SwapCardLeft,
    SwapCardRight,
    AcceptDeckChanges,
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::Restart, GamepadButtonType::Select)
                .insert(Self::Restart, KeyCode::KeyR)
                .insert(Self::TogglePause, GamepadButtonType::Start)
                .insert(Self::TogglePause, KeyCode::Escape)
                .insert(Self::TogglePause, KeyCode::Tab)
                .insert(Self::TogglePause, KeyCode::KeyP)
                .insert(Self::SelectCardLeft, GamepadButtonType::DPadLeft)
                .insert(Self::SelectCardLeft, GamepadButtonType::LeftTrigger)
                .insert(Self::SelectCardLeft, KeyCode::KeyA)
                .insert(Self::SelectCardLeft, KeyCode::ArrowLeft)
                .insert(Self::SelectCardRight, GamepadButtonType::DPadRight)
                .insert(Self::SelectCardRight, GamepadButtonType::RightTrigger)
                .insert(Self::SelectCardRight, KeyCode::KeyD)
                .insert(Self::SelectCardRight, KeyCode::ArrowRight)
                .insert(Self::SwapCardLeft, GamepadButtonType::LeftTrigger2)
                .insert_modified(Self::SwapCardLeft, Modifier::Shift, KeyCode::KeyA)
                .insert_modified(Self::SwapCardLeft, Modifier::Shift, KeyCode::ArrowLeft)
                .insert(Self::SwapCardRight, GamepadButtonType::RightTrigger2)
                .insert_modified(Self::SwapCardRight, Modifier::Shift, KeyCode::KeyD)
                .insert_modified(Self::SwapCardRight, Modifier::Shift, KeyCode::ArrowRight)
                .insert(Self::AcceptDeckChanges, KeyCode::Enter)
                .insert(Self::AddCard, KeyCode::KeyL)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            (
                restart.in_set(ResolveStateSet::<Screen>::Compute).run_if(
                    Screen::Playing
                        .will_exit()
                        .and_then(action_just_pressed(Self::Restart)),
                ),
                PlayingMenu::Pause
                    .toggle()
                    .in_set(ResolveStateSet::<PlayingMenu>::Compute)
                    .run_if(
                        Screen::Playing
                            .will_exit()
                            .and_then(action_just_pressed(Self::TogglePause)),
                    ),
            ),
        );
    }
}

fn restart(mut commands: Commands) {
    commands.spawn_with(fade_out(Screen::Playing));
}

// TODO: Where can we define the in-game pause menu?
// In playing screen, we can say "on TogglePause action, toggle the Pause state AND toggle the in-game pause menu"
// In-game pause menu should be defined by the playing screen itself. Think of it like an extension of the HUD, but usually hidden.
// It _could_ be in a submodule as well.
// Then what about the level-up menu? Is that also defined by the playing screen?
// What happens if you try to pause while in the level-up menu?

// TODO: This state is usually disabled. Disable it when you exit `Screen::Playing`. Also make sure `PlayingAction` is enabled / disabled based on `Screen::Playing`.
//       on `PlayingAction::Pause`, enter `PlayingMenu::Pause` which will enable `Pause`. Exiting `PlayingMenu::Pause` will disable `Pause`.
//       Same pausing behavior for `PlayingMenu::LevelUp`.
//       Use state-scoping for any `PlayingMenu` spawned UI, while also being a child of the UI root (not the playing screen). Compare this to how the playing screen root UI node is set up.
//       Both playing menus will be defined in `src/screen/playing/`, not in `src/game/actor/level/up` or whatever.
#[derive(State, Eq, PartialEq, Clone, Debug, Reflect)]
#[state(after(Screen), before(Pause), entity_scope, log_flush)]
#[reflect(Resource)]
enum PlayingMenu {
    Pause,
    LevelUp,
}

impl Configure for PlayingMenu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(StateFlush, Screen::Playing.on_exit(Self::disable));
    }
}
