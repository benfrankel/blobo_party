use bevy::math::vec3;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use pyri_state::schedule::ResolveStateSet;

use crate::core::camera::CameraRoot;
use crate::game::actor::enemy::enemy;
use crate::game::actor::player::player;
use crate::game::level::xp::PlayerXp;
use crate::game::level::PlayerLevel;
use crate::game::level::PlayerLevelIndicator;
use crate::game::GameRoot;
use crate::screen::fade_in;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        Screen::Playing.on_edge(exit_playing, enter_playing),
    );

    app.configure::<(PlayingAssets, PlayingAction)>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayingAssets {}

impl Configure for PlayingAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn enter_playing(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    // TODO: Character select screen.
    commands.spawn_with(player("pink"));
    // TODO: Enemy spawner from config.
    commands
        .spawn_with(enemy("blue"))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(vec3(20.0, 0.0, 0.0)),
        ));
    commands
        .spawn_with(enemy("red"))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(vec3(20.0, 0.0, 0.0)),
        ));
    commands
        .spawn_with(enemy("green"))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(vec3(20.0, 0.0, 0.0)),
        ));
    commands
        .spawn_with(enemy("purple"))
        .insert(TransformBundle::from_transform(
            Transform::from_translation(vec3(20.0, 0.0, 0.0)),
        ));

    commands.spawn_with(playing_hud).set_parent(ui_root.body);
}

fn exit_playing(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    game_root: Res<GameRoot>,
    camera_root: Res<CameraRoot>,
    mut camera_query: Query<&mut Transform>,
) {
    // Reset resources
    commands.insert_resource(PlayerLevel::default());
    commands.insert_resource(PlayerXp::default());

    // Clear events

    // Despawn entities
    commands.entity(ui_root.body).despawn_descendants();
    commands.entity(game_root.players).despawn_descendants();
    commands.entity(game_root.enemies).despawn_descendants();
    commands.entity(game_root.projectiles).despawn_descendants();

    // Reset camera
    if let Ok(mut transform) = camera_query.get_mut(camera_root.primary) {
        transform.translation = Vec2::ZERO.extend(transform.translation.z);
    };
}

fn playing_hud(mut entity: EntityWorldMut) {
    entity
        .add(widget::column_left)
        .insert(Name::new("PlayingScreen"))
        .with_children(|children| {
            children.spawn_with(level_hud);
        });
}

fn level_hud(mut entity: EntityWorldMut) {
    entity
        .insert((
            Name::new("LevelHud"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    align_items: default(),
                    justify_content: default(),
                    padding: UiRect::all(Px(8.0)),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn_with(level_indicator);
            children.spawn_with(xp_bar);
        });
}

fn level_indicator(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("LevelIndicator"),
        TextBundle::from_section(
            "",
            TextStyle {
                font: FONT_HANDLE,
                font_size: 32.0,
                ..default()
            },
        ),
        ThemeColorForText(vec![ThemeColor::Indicator]),
        PlayerLevelIndicator,
    ));
}

// TODO
fn xp_bar(mut entity: EntityWorldMut) {
    entity.insert((
        Name::new("XpBar"),
        NodeBundle {
            style: Style {
                width: Percent(100.0),
                ..default()
            },
            ..default()
        },
    ));
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum PlayingAction {
    Restart,
    RotateDock,
    AddCard,
    // TODO: Pause
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::Restart, KeyCode::KeyR)
                .insert(Self::RotateDock, KeyCode::BracketLeft)
                .insert(Self::AddCard, KeyCode::KeyL)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            Screen::refresh
                .in_set(ResolveStateSet::<Screen>::Compute)
                .run_if(
                    Screen::Playing
                        .will_exit()
                        .and_then(action_just_pressed(Self::Restart)),
                ),
        );
    }
}
