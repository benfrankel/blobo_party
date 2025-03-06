//! Debugging tools for dev builds.

mod stepping;

use avian2d::prelude::*;
use bevy::core::FrameCount;
use bevy::dev_tools::ui_debug_overlay::DebugUiPlugin;
use bevy::dev_tools::ui_debug_overlay::UiDebugOptions;
// TODO: This will exist in bevy 0.16.
//use bevy::dev_tools::picking_debug::DebugPickingMode;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::LogLevel;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::core::window::WindowReady;
use crate::screen::Screen;
use crate::screen::wait;

pub(super) fn plugin(app: &mut App) {
    // TODO: Load from file.
    let config = DebugConfig::default();

    // Collect diagnostics.
    if config.frame_time_diagnostics {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
    }
    if config.system_information_diagnostics {
        app.add_plugins(SystemInformationDiagnosticsPlugin);
    }
    if config.entity_count_diagnostics {
        app.add_plugins(EntityCountDiagnosticsPlugin);
    }

    // Log diagnostics.
    if config.log_diagnostics {
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    // Log ambiguity detection results.
    if config.log_ambiguity_detection {
        for (_, schedule) in app.world_mut().resource_mut::<Schedules>().iter_mut() {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        }
    }

    // Log state flushes.
    app.insert_resource(StateDebugSettings {
        log_flush: config.log_state_flush,
        ..default()
    });

    // Set up system stepping.
    if config.system_stepping {
        app.add_plugins(
            stepping::SteppingPlugin::default()
                .add_schedule(PreUpdate)
                .add_schedule(PostUpdate),
        );
    }

    // Debug picking.
    if config.debug_picking {
        // TODO: This will exist in bevy 0.16.
        let _ = 0;
        /*app.add_systems(
            Update,
            toggle_debug_picking.run_if(input_just_pressed(DEBUG_TOGGLE_KEY)),
        );

        fn toggle_debug_picking(mut mode: ResMut<DebugPickingMode>) {
            *mode = match *mode {
                DebugPickingMode::Disabled => DebugPickingMode::Normal,
                _ => DebugPickingMode::Disabled,
            };
        }*/
    }

    // Debug UI.
    if config.debug_ui {
        app.add_plugins(DebugUiPlugin);
        app.add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(DEBUG_TOGGLE_KEY)),
        );

        fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
            options.toggle();
        }
    }

    // Debug physics.
    if config.debug_physics {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.world_mut()
            .resource_mut::<GizmoConfigStore>()
            .config_mut::<PhysicsGizmos>()
            .0
            .enabled = false;
        app.add_systems(
            Update,
            (|mut gizmos: ResMut<GizmoConfigStore>| {
                gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
            })
            .run_if(input_just_pressed(DEBUG_TOGGLE_KEY)),
        );
    }

    // Enable editor.
    if config.editor {
        app.add_plugins(EditorPlugin::new().in_new_window(Window {
            title: "bevy_editor_pls".to_string(),
            focused: false,
            ..default()
        }));
    }

    // Extend loading screen.
    if config.extend_loading_screen > 0.0 {
        app.add_systems(
            Update,
            (
                Screen::Title
                    .on_update((|| Progress::from(false)).track_progress::<BevyState<Screen>>()),
                Screen::Loading.on_update(wait(config.extend_loading_screen)),
            ),
        );
    }

    // Skip to a custom starting screen.
    if let Some(start_screen) = config.start_screen {
        // Setting this later avoids a plugin ordering requirement.
        app.add_systems(StateFlush, WindowReady.on_enter(start_screen.enter()));
    }

    // Set up ad hoc debugging.
    app.add_systems(Update, debug_start);
    app.add_systems(Update, debug_end);
}

pub struct DebugConfig {
    // Diagnostics
    pub frame_time_diagnostics: bool,
    pub system_information_diagnostics: bool,
    pub entity_count_diagnostics: bool,

    // Logging
    pub log_diagnostics: bool,
    pub log_ambiguity_detection: bool,
    pub log_state_flush: bool,

    // Debug tools
    pub debug_picking: bool,
    pub debug_ui: bool,
    pub debug_physics: bool,
    pub system_stepping: bool,
    pub editor: bool,

    // Screen settings
    pub start_screen: Option<Screen>,
    pub extend_loading_screen: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            frame_time_diagnostics: true,
            system_information_diagnostics: true,
            entity_count_diagnostics: true,

            log_diagnostics: false,
            log_ambiguity_detection: false,
            log_state_flush: true,

            debug_picking: true,
            debug_ui: true,
            debug_physics: true,
            system_stepping: false,
            editor: true,

            extend_loading_screen: 0.0,
            start_screen: Some(Screen::Playing),
        }
    }
}

const DEBUG_TOGGLE_KEY: KeyCode = KeyCode::F3;

fn debug_start(world: &mut World) {
    let frame = world.resource::<FrameCount>().0;
    let prefix = format!("[Frame {frame} start] ");
    let _ = prefix;
}

fn debug_end(world: &mut World) {
    let frame = world.resource::<FrameCount>().0;
    let prefix = format!("[Frame {frame} end] ");
    let _ = prefix;
}
