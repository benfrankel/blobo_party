use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::animation::backup::Backup;
use crate::animation::offset::Offset;
use crate::ui::prelude::*;

pub fn overlay(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style::ABS_FILL,
        z_index: ZIndex::Global(1000),
        ..default()
    });
}

pub fn blocking_overlay(mut entity: EntityWorldMut) {
    entity.insert(NodeBundle {
        style: Style::ABS_FILL,
        focus_policy: FocusPolicy::Block,
        z_index: ZIndex::Global(1000),
        ..default()
    });
}

pub fn menu_button(text: impl Into<String>) -> impl EntityCommand<World> {
    menu_button_with_font_size(text, Vw(4.0))
}

pub fn menu_button_with_font_size(
    text: impl Into<String>,
    font_size: Val,
) -> impl EntityCommand<World> {
    let text = text.into();
    move |mut entity: EntityWorldMut| {
        entity
            .insert((
                Name::new(format!("Button(\"{}\")", text)),
                ButtonBundle {
                    style: Style {
                        height: Vw(11.0),
                        width: Vw(38.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                ThemeColor::default().target::<BackgroundColor>(),
                InteractionTable {
                    normal: ThemeColor::Primary.target::<BackgroundColor>(),
                    hovered: ThemeColor::PrimaryHovered.target::<BackgroundColor>(),
                    pressed: ThemeColor::PrimaryPressed.target::<BackgroundColor>(),
                    disabled: ThemeColor::PrimaryDisabled.target::<BackgroundColor>(),
                },
                Offset::default(),
                Backup::<Transform>::default(),
                InteractionTable {
                    hovered: Offset(Vec2::new(0.0, -4.0)),
                    pressed: Offset(Vec2::new(0.0, 2.0)),
                    ..default()
                },
                InteractionSfx,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Name::new("ButtonText"),
                    TextBundle::from_section(
                        text,
                        TextStyle {
                            font: FONT_HANDLE,
                            ..default()
                        },
                    ),
                    DynamicFontSize::new(font_size).with_step(8.0),
                    ThemeColorForText(vec![ThemeColor::PrimaryText]),
                ));
            });
    }
}
