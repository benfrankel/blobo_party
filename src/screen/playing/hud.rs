use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

use crate::game::actor::level::IsLevelDisplayCurrent;
use crate::game::actor::level::xp::IsXpBarFill;
use crate::game::card::CardConfig;
use crate::game::card::deck::IsDeckDisplay;
use crate::screen::playing::PlayingAssets;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn playing_hud(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("PlayingScreen"),
                Node {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
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
                Node {
                    width: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: default(),
                    padding: UiRect::all(Px(16.0)),
                    column_gap: Px(16.0),
                    ..default()
                },
                GlobalZIndex(2),
            ))
            .with_children(|children| {
                children.spawn_with(level_display(player));
                children.spawn_with(xp_bar(player));
            });
    }
}

fn level_display(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("LevelDisplay"),
                Text::new("Level "),
                TextFont {
                    font: FONT_HANDLE,
                    font_size: 32.0,
                    ..default()
                },
                ThemeColor::Indicator.target::<TextColor>(),
                TextLayout::new_with_no_wrap(),
                Node {
                    margin: UiRect::new(Val::ZERO, Px(-4.0), Px(-4.0), Val::ZERO),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("LevelDisplayCurrent"),
                    TextSpan::default(),
                    TextFont {
                        font: FONT_HANDLE,
                        font_size: 32.0,
                        ..default()
                    },
                    ThemeColor::Indicator.target::<TextColor>(),
                    IsLevelDisplayCurrent,
                    Selection(player),
                ));
                parent.spawn((
                    Name::new("LevelDisplayVictory"),
                    TextSpan::new("/10"),
                    TextFont {
                        font: FONT_HANDLE,
                        font_size: 32.0,
                        ..default()
                    },
                    ThemeColor::Indicator.target::<TextColor>(),
                ));
            });
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
                ImageNode::from(texture).with_mode(NodeImageMode::Sliced(TextureSlicer {
                    border: BorderRect::square(8.0),
                    ..default()
                })),
                ThemeColor::Indicator.target::<ImageNode>(),
                Node {
                    width: Percent(100.0),
                    height: Px(28.0),
                    //padding: UiRect::all(Px(8.0)),
                    // TODO: Why is this needed? Bevy layouting bug?
                    margin: UiRect::right(Px(4.0)),
                    ..default()
                },
            ))
            .with_children(|children| {
                // TODO: Workaround for padding not working in UI images.
                children
                    .spawn((
                        Name::new("XpBarPaddingWorkaround"),
                        Node {
                            width: Percent(100.0),
                            height: Percent(100.0),
                            padding: UiRect::all(Px(8.0)),
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
            Node {
                height: Percent(100.0),
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
        .queue(Node::ROW_TOP.div())
        .insert(Name::new("MiddleHud"));
}

fn lower_hud(player: Entity) -> impl EntityCommand<World> {
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new("LowerHud"),
                Node {
                    width: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                GlobalZIndex(2),
            ))
            .with_children(|children| {
                children.spawn_with(arrow);
                children.spawn_with(deck_display(player));
                children.spawn_with(arrow);
            });
    }
}

fn deck_display(player: Entity) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
        let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
        let config = r!(config.get());
        let column_gap = config.card_height / 18.0 * -1.0;

        world.entity_mut(entity).insert((
            Name::new("DeckDisplay"),
            Node {
                column_gap,
                ..default()
            },
            IsDeckDisplay,
            Selection(player),
        ));
    }
}

fn arrow(entity: Entity, world: &mut World) {
    let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
    let config = r!(config.get());
    let height = config.card_height / 18.0 * 5.0;
    let margin = config.card_height / 18.0;
    let texture = world.resource::<PlayingAssets>().arrow.clone();

    world.entity_mut(entity).insert((
        Name::new("Arrow"),
        ImageNode::from(texture),
        ThemeColor::Indicator.target::<ImageNode>(),
        Node {
            height,
            margin: UiRect::horizontal(margin),
            ..default()
        },
    ));
}
