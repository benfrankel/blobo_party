use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use pyri_state::schedule::ResolveStateSet;

use crate::core::camera::CameraRoot;
use crate::core::pause::Pause;
use crate::game::actor::level::xp::IsXpBarFill;
use crate::game::actor::level::IsLevelIndicator;
use crate::game::actor::player::player;
use crate::game::card::deck::deck_display;
use crate::game::spotlight::spotlight_lamp_spawner;
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
    Pause,
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
                .insert(Self::Pause, GamepadButtonType::Start)
                .insert(Self::Pause, KeyCode::Escape)
                .insert(Self::Pause, KeyCode::Tab)
                .insert(Self::Pause, KeyCode::KeyP)
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
                Screen::refresh
                    .in_set(ResolveStateSet::<Screen>::Compute)
                    .run_if(
                        Screen::Playing
                            .will_exit()
                            .and_then(action_just_pressed(Self::Restart)),
                    ),
                Pause::toggle_default
                    .in_set(ResolveStateSet::<Pause>::Compute)
                    .run_if(
                        Screen::Playing
                            .will_exit()
                            .and_then(action_just_pressed(Self::Pause)),
                    ),
            ),
        );
    }
}

fn exit_playing(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    game_root: Res<GameRoot>,
    camera_root: Res<CameraRoot>,
    mut camera_query: Query<&mut Transform>,
) {
    // Reset resources

    // Clear events

    // Despawn entities
    commands.entity(ui_root.body).despawn_descendants();
    game_root.despawn_descendants(&mut commands);

    // Reset camera
    if let Ok(mut transform) = camera_query.get_mut(camera_root.primary) {
        transform.translation = Vec2::ZERO.extend(transform.translation.z);
    };
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

fn playing_hud(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("PlayingScreen"),
                NodeBundle {
                    style: Style {
                        width: Percent(100.0),
                        height: Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn_with(upper_hud(player));
                children.spawn_with(middle_hud);
                children.spawn_with(lower_hud(player));
            });
    }
}

fn upper_hud(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("UpperHud"),
                NodeBundle {
                    style: Style {
                        width: Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: default(),
                        padding: UiRect::all(Px(16.0)),
                        column_gap: Px(16.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn_with(level_indicator(player));
                children.spawn_with(xp_bar(player));
            });
    }
}

fn level_indicator(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.insert((
            Name::new("LevelIndicator"),
            TextBundle::from_section(
                "",
                TextStyle {
                    font: FONT_HANDLE,
                    font_size: 32.0,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::new(Val::ZERO, Px(-4.0), Px(-4.0), Val::ZERO),
                ..default()
            }),
            ThemeColorForText(vec![ThemeColor::Indicator]),
            IsLevelIndicator,
            Selection(player),
        ));
    }
}

fn xp_bar(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        let texture = entity
            .world()
            .resource::<PlayingAssets>()
            .simple_border
            .clone();

        entity
            .insert((
                Name::new("XpBar"),
                ImageBundle {
                    style: Style {
                        width: Percent(100.0),
                        height: Px(28.0),
                        //padding: UiRect::all(Px(8.0)),
                        // TODO: Why is this needed? Bevy layouting bug?
                        margin: UiRect::right(Px(4.0)),
                        ..default()
                    },
                    image: UiImage::new(texture),
                    ..default()
                },
                ImageScaleMode::Sliced(TextureSlicer {
                    border: BorderRect::square(8.0),
                    ..default()
                }),
                ThemeColor::Indicator.target::<UiImage>(),
            ))
            .with_children(|children| {
                // TODO: Workaround for padding not working in UI images.
                children
                    .spawn((
                        Name::new("XpBarPaddingWorkaround"),
                        NodeBundle {
                            style: Style {
                                width: Percent(100.0),
                                height: Percent(100.0),
                                padding: UiRect::all(Px(8.0)),
                                ..default()
                            },
                            ..default()
                        },
                    ))
                    .with_children(|children| {
                        children.spawn_with(xp_bar_fill(player));
                    });
            });
    }
}

fn xp_bar_fill(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity.insert((
            Name::new("XpBarFill"),
            NodeBundle {
                style: Style {
                    height: Percent(100.0),
                    ..default()
                },
                ..default()
            },
            ThemeColor::Indicator.target::<BackgroundColor>(),
            IsXpBarFill,
            Selection(player),
        ));
    }
}

fn middle_hud(mut entity: EntityWorldMut) {
    entity.add(widget::row_top).insert(Name::new("MiddleHud"));
}

fn lower_hud(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("LowerHud"),
                NodeBundle {
                    style: Style {
                        width: Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn_with(deck_display(player));
            });
    }
}
