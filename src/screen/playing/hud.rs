use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::game::actor::level::xp::IsXpBarFill;
use crate::game::actor::level::IsLevelIndicator;
use crate::game::card::deck::deck_display;
use crate::screen::playing::PlayingAssets;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn playing_hud(player: Entity) -> impl EntityCommand<World> {
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
    entity
        .add(Style::ROW_TOP.div())
        .insert(Name::new("MiddleHud"));
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