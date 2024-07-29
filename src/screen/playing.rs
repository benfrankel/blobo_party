pub mod defeat_menu;
pub mod hud;
pub mod level_up_menu;
pub mod pause_menu;
pub mod victory_menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use pyri_state::schedule::ResolveStateSet;

use crate::core::pause::Pause;
use crate::game::actor::player::player;
use crate::game::audio::music::start_music;
use crate::game::audio::music::stop_music;
use crate::game::ground::ground;
use crate::game::spotlight::spotlight_lamp_spawner;
use crate::game::stats::Stats;
use crate::game::wave::wave;
use crate::game::GameRoot;
use crate::screen::fade_in;
use crate::screen::playing::hud::playing_hud;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(stop_music, (enter_playing, start_music)),
    );

    app.configure::<(PlayingAssets, PlayingAction, PlayingMenu)>();

    app.add_plugins((
        level_up_menu::plugin,
        pause_menu::plugin,
        victory_menu::plugin,
        defeat_menu::plugin,
    ));
}

fn enter_playing(
    mut commands: Commands,
    game_root: Res<GameRoot>,
    ui_root: Res<UiRoot>,
    mut stats: ResMut<Stats>,
) {
    commands.spawn_with(fade_in);

    // Reset stats.
    *stats = default();

    // TODO: Character select screen.
    // Spawn player.
    let player = commands.spawn_with(player("pink")).id();

    // Spawn enemies.
    commands
        .spawn_with(wave(player))
        .set_parent(game_root.enemies);

    // Spawn UI.
    commands
        .spawn_with(playing_hud(player))
        .set_parent(ui_root.body);

    // Spawn background.
    commands.spawn_with(ground).set_parent(game_root.background);

    // Spawn VFX.
    commands
        .spawn_with(spotlight_lamp_spawner)
        .set_parent(game_root.vfx);
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

    #[asset(path = "audio/music/Menu Theme.ogg")]
    pub music: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Restart_1.ogg")]
    pub sfx_restart: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Level Up_1.ogg")]
    pub sfx_level_up: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Projectile Hits Player-02.ogg")]
    pub sfx_hurt: Handle<AudioSource>,
    #[asset(path = "audio/sfx/UI Hover.ogg")]
    pub sfx_ui_click: Handle<AudioSource>,
    #[asset(path = "audio/sfx/UI Click.ogg")]
    pub sfx_ui_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/Movement.ogg")]
    pub sfx_movement: Handle<AudioSource>,

    // CC0 assets from freesound.org:
    #[asset(path = "audio/sfx/24787__young_daddy__clap-mix2.ogg")]
    pub sfx_clap: Handle<AudioSource>,
    #[asset(path = "audio/sfx/102130__noirpantalon__hard_subby_kick.ogg")]
    pub sfx_kick: Handle<AudioSource>,
    #[asset(path = "audio/sfx/137105__chaosportal__finger-snap-01.ogg")]
    pub sfx_snap0: Handle<AudioSource>,
    #[asset(path = "audio/sfx/444407__mootmcnoodles__slap.ogg")]
    pub sfx_snap1: Handle<AudioSource>,
}

impl Configure for PlayingAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum PlayingAction {
    TogglePause,
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::TogglePause, GamepadButtonType::Start)
                .insert(Self::TogglePause, KeyCode::Escape)
                .insert(Self::TogglePause, KeyCode::Tab)
                .insert(Self::TogglePause, KeyCode::KeyP)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            (
                Screen::Playing.on_exit(PlayingMenu::disable),
                PlayingMenu::Pause
                    .toggle()
                    .in_set(ResolveStateSet::<PlayingMenu>::Compute)
                    .run_if(
                        PlayingMenu::is_disabled
                            .or_else(PlayingMenu::Pause.will_exit())
                            .and_then(Screen::Playing.will_enter())
                            .and_then(action_just_pressed(Self::TogglePause)),
                    ),
            ),
        );
    }
}

#[derive(State, Eq, PartialEq, Clone, Debug, Reflect)]
#[state(after(Screen), before(Pause), entity_scope, log_flush)]
#[reflect(Resource)]
pub enum PlayingMenu {
    Pause,
    LevelUp,
    Victory,
    Defeat,
}

impl Configure for PlayingMenu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(StateFlush, Screen::Playing.on_exit(Self::disable));
    }
}
