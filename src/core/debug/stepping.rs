//! Adapted from https://github.com/bevyengine/bevy/blob/main/examples/games/stepping.rs

use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::*;
use bevy::prelude::*;
use disqualified::ShortName;
use tiny_bail::prelude::*;

/// A plugin to add stepping UI.
#[derive(Default)]
pub struct SteppingPlugin {
    schedule_labels: Vec<InternedScheduleLabel>,
}

impl SteppingPlugin {
    /// Add a schedule to be paused and stepped when stepping is enabled.
    pub fn add_schedule(mut self, label: impl ScheduleLabel) -> SteppingPlugin {
        self.schedule_labels.push(label.intern());
        self
    }
}

impl Plugin for SteppingPlugin {
    fn build(&self, app: &mut App) {
        info!("Press ` to toggle stepping mode (S: step system, Space: step frame)");

        // create and insert our debug schedule into the main schedule order.
        // We need an independent schedule so we have access to all other
        // schedules through the `Stepping` resource
        app.init_schedule(SteppingSchedule);
        let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
        order.insert_after(Update, SteppingSchedule);

        // create our stepping resource
        let mut stepping = Stepping::new();
        for label in &self.schedule_labels {
            stepping.add_schedule(*label);
        }
        app.insert_resource(stepping);

        // add our startup & stepping systems
        app.init_resource::<SteppingContext>().add_systems(
            SteppingSchedule,
            (
                build_stepping_ui.run_if(not(is_stepping_initialized)),
                handle_stepping_input,
                update_stepping_ui.run_if(is_stepping_initialized),
            )
                .chain(),
        );
    }
}

const FONT_SIZE: f32 = 12.0;
const FONT_COLOR_SCHEDULE: Color = Color::srgb(0.8, 0.9, 0.9);
const FONT_COLOR_SYSTEM: Color = Color::srgb(0.5, 0.6, 0.6);
const FONT_COLOR_SYSTEM_NEXT: Color = Color::srgb(0.1, 0.9, 0.8);
const FONT_COLOR_SYSTEM_ALWAYS_RUN: Color = Color::srgb(0.4, 0.6, 0.4);
const FONT_COLOR_SYSTEM_NEVER_RUN: Color = Color::srgb(0.6, 0.4, 0.4);

/// A separate [`Schedule`] for stepping plugin systems.
#[derive(Debug, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
struct SteppingSchedule;

/// The stepping context for a specific system.
#[derive(Default, Debug)]
struct SystemSteppingContext {
    text_idx_mark: usize,
    text_idx_name: usize,
    always_run: bool,
    never_run: bool,
    breakpoint: bool,
}

impl SystemSteppingContext {
    fn new(text_mark_idx: usize) -> Self {
        Self {
            text_idx_mark: text_mark_idx,
            text_idx_name: text_mark_idx + 1,
            always_run: false,
            never_run: false,
            breakpoint: false,
        }
    }
}

/// A resource to keep track of stepping plugin state.
#[derive(Resource, Default, Debug)]
struct SteppingContext {
    /// A list of mappings from (schedule, system) -> stepping context for that system.
    systems: Vec<(InternedScheduleLabel, NodeId, SystemSteppingContext)>,

    // UI positioning.
    ui_top: Val,
    ui_left: Val,
}

impl SteppingContext {
    fn next_system_idx(&self, stepping: &Stepping) -> Option<usize> {
        let (schedule, node_id) = rq!(stepping.cursor());

        for (idx, &(ctx_schedule, ctx_node_id, _)) in self.systems.iter().enumerate() {
            if ctx_schedule == schedule && ctx_node_id == node_id {
                return Some(idx);
            }
        }

        None
    }
}

/// A run condition to check if the stepping UI has been constructed.
fn is_stepping_initialized(ctx: Res<SteppingContext>) -> bool {
    !ctx.systems.is_empty()
}

/// Returns true if a system should be ignored by stepping and always run.
fn should_always_run(system: &Box<dyn System<In = (), Out = ()>>) -> bool {
    let name = system.name();
    name.starts_with("bevy") || name.starts_with("Pipe") || name.starts_with("avian2d")
}

#[derive(Component)]
struct IsSteppingUi;

/// Construct the stepping UI elements from the [`Schedules`] resource.
///
/// This system may run multiple times before constructing the UI as all of the
/// data may not be available on the first run of the system.  This happens if
/// one of the stepping schedules has not yet been run.
fn build_stepping_ui(
    mut commands: Commands,
    schedules: Res<Schedules>,
    mut stepping: ResMut<Stepping>,
    mut ctx: ResMut<SteppingContext>,
) {
    let mut text_spans = Vec::new();
    let mut always_run = Vec::new();

    for &label in rq!(stepping.schedules()) {
        text_spans.push((
            Name::new(format!("Schedule({label:?})")),
            TextSpan(format!("{label:?}\n")),
            TextFont::from_font_size(FONT_SIZE),
            TextColor(FONT_COLOR_SCHEDULE),
        ));

        for (node_id, system) in cq!(schedules.get(label).and_then(|x| x.systems().ok())) {
            if should_always_run(system) {
                always_run.push((label, node_id));
                continue;
            }

            ctx.systems.push((
                label,
                node_id,
                SystemSteppingContext::new(text_spans.len() + 1),
            ));

            // Add a text section for displaying the cursor for this system.
            text_spans.push((
                Name::new("SteppingCursor"),
                TextSpan::new("   "),
                TextFont::from_font_size(FONT_SIZE),
                TextColor(FONT_COLOR_SYSTEM),
            ));

            // Add the name of the system to the UI.
            text_spans.push((
                Name::new("SystemName"),
                TextSpan(format!("{}\n", ShortName::from(&*system.name()))),
                TextFont::from_font_size(FONT_SIZE),
                TextColor(FONT_COLOR_SYSTEM),
            ));
        }
    }

    for (schedule, node_id) in always_run.drain(..) {
        stepping.always_run_node(schedule, node_id);
    }

    commands
        .spawn((
            Text::default(),
            TextLayout::new_with_no_wrap(),
            Node {
                position_type: PositionType::Absolute,
                top: ctx.ui_top,
                left: ctx.ui_left,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            GlobalZIndex(i32::MAX),
            BackgroundColor(Color::BLACK),
            Visibility::Hidden,
            IsSteppingUi,
        ))
        .with_children(|p| {
            for span in text_spans {
                p.spawn(span);
            }
        });
}

fn handle_stepping_input(
    key: Res<ButtonInput<KeyCode>>,
    mut stepping: ResMut<Stepping>,
    mut ctx: ResMut<SteppingContext>,
) {
    // Debug stepping state.
    if key.just_pressed(KeyCode::Slash) {
        info!("{:#?}", stepping);
    }

    // Toggle stepping mode.
    if key.just_pressed(KeyCode::Backquote) {
        if stepping.is_enabled() {
            stepping.disable();
            info!("Disabled stepping");
        } else {
            stepping.enable();
        }
    }

    if !stepping.is_enabled() {
        return;
    }

    // Advance the frame.
    if key.just_pressed(KeyCode::Space) {
        stepping.continue_frame();
    } else if key.just_pressed(KeyCode::KeyS) {
        stepping.step_frame();
    }

    // Mark the next system.
    let system_idx = rq!(ctx.next_system_idx(&stepping));
    let (schedule, node_id, ref mut system) = ctx.systems[system_idx];
    if key.just_pressed(KeyCode::KeyQ) {
        stepping.never_run_node(schedule, node_id);
        system.always_run = false;
        system.never_run = true;
    } else if key.just_pressed(KeyCode::KeyW) {
        stepping.clear_node(schedule, node_id);

        // Re-set breakpoint.
        if system.breakpoint {
            stepping.set_breakpoint_node(schedule, node_id);
        }
    } else if key.just_pressed(KeyCode::KeyE) {
        stepping.always_run_node(schedule, node_id);
        system.always_run = true;
        system.never_run = false;
    } else if key.just_pressed(KeyCode::KeyA) {
        stepping.clear_breakpoint_node(schedule, node_id);
        system.breakpoint = false;
    } else if key.just_pressed(KeyCode::KeyD) {
        stepping.set_breakpoint_node(schedule, node_id);
        system.breakpoint = true;

        // Re-set other behaviors.
        if system.always_run {
            stepping.always_run_node(schedule, node_id);
        }
        if system.never_run {
            stepping.never_run_node(schedule, node_id);
        }
    }
}

fn update_stepping_ui(
    ctx: Res<SteppingContext>,
    stepping: Res<Stepping>,
    mut stepping_ui_query: Query<(Entity, &mut Visibility), With<IsSteppingUi>>,
    mut writer: TextUiWriter,
) {
    for (entity, mut visibility) in &mut stepping_ui_query {
        // Update visibility.
        match (stepping.is_enabled(), *visibility) {
            (true, Visibility::Hidden) => {
                *visibility = Visibility::Inherited;
            },
            (false, Visibility::Inherited | Visibility::Visible) => {
                *visibility = Visibility::Hidden;
            },
            _ => (),
        }

        cq!(stepping.is_enabled());

        // Update text.
        let next_system_idx = cq!(ctx.next_system_idx(&stepping));
        for (idx, (_, _, system)) in ctx.systems.iter().enumerate() {
            let (mark, color) = if idx == next_system_idx {
                (
                    if system.breakpoint { "-@ " } else { "-> " },
                    FONT_COLOR_SYSTEM_NEXT,
                )
            } else if system.always_run {
                (" + ", FONT_COLOR_SYSTEM_ALWAYS_RUN)
            } else if system.never_run {
                (" - ", FONT_COLOR_SYSTEM_NEVER_RUN)
            } else if system.breakpoint {
                (" @ ", FONT_COLOR_SYSTEM)
            } else {
                ("   ", FONT_COLOR_SYSTEM)
            };
            *writer.text(entity, system.text_idx_mark) = mark.to_string();
            *writer.color(entity, system.text_idx_mark) = color.into();
            *writer.color(entity, system.text_idx_name) = color.into();
        }
    }
}
