use bevy::prelude::*;

use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let container = app
        .world_mut()
        .spawn((
            Name::new("PrimaryTooltip"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    max_width: Vw(40.0),
                    padding: UiRect::all(Px(8.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(999),
                ..default()
            },
            ThemeColor::Popup.target::<BackgroundColor>(),
        ))
        .id();
    let text = app
        .world_mut()
        .spawn((
            Name::new("Text"),
            TextBundle::default(),
            DynamicFontSize::new(Px(16.0)),
            ThemeColorForText(vec![ThemeColor::BodyText]),
        ))
        .set_parent(container)
        .id();

    app.add_plugins(TooltipPlugin { container, text });
}
