use bevy::prelude::*;

use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let container = app
        .world_mut()
        .spawn((
            Name::new("PrimaryTooltip"),
            Node {
                position_type: PositionType::Absolute,
                padding: UiRect::all(Px(8.0)),
                ..default()
            },
            ThemeColor::Popup.target::<BackgroundColor>(),
            Visibility::Hidden,
            GlobalZIndex(999),
        ))
        .id();
    let text = app
        .world_mut()
        .spawn((
            Name::new("Text"),
            RichText::default(),
            DynamicFontSize::new(Px(16.0)),
            ThemeColor::BodyText.target::<TextColor>(),
        ))
        .set_parent(container)
        .id();

    app.add_plugins(TooltipPlugin { container, text });
}
