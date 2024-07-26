use bevy::prelude::*;

use crate::core::window::WindowRoot;
use crate::core::UpdateSet;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(TooltipRoot, Tooltip)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TooltipRoot {
    pub container: Entity,
    pub text: Entity,
}

impl Configure for TooltipRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for TooltipRoot {
    fn from_world(world: &mut World) -> Self {
        let container = world
            .spawn((
                Name::new("Tooltip"),
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

        Self {
            container,
            text: world
                .spawn((
                    Name::new("TooltipText"),
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: FONT_HANDLE,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center),
                    // TODO: Adjustable font sizes in ThemeConfig
                    DynamicFontSize::new(Px(16.0)),
                    ThemeColorForText(vec![ThemeColor::BodyText]),
                ))
                .set_parent(container)
                .id(),
        }
    }
}

#[derive(Reflect)]
pub enum TooltipSide {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tooltip {
    pub text: String,
    pub side: TooltipSide,
    // TODO: Val
    pub offset: Vec2,
}

impl Configure for Tooltip {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, show_tooltip_on_hover.in_set(UpdateSet::RecordInput));
    }
}

// TODO: Set text in an early system, then set position in a late system.
//       That way the tooltip can use its own calculated size to support centering.
fn show_tooltip_on_hover(
    window_root: Res<WindowRoot>,
    window_query: Query<&Window>,
    tooltip_root: Res<TooltipRoot>,
    mut container_query: Query<(&mut Visibility, &mut Style)>,
    mut text_query: Query<&mut Text>,
    interaction_query: Query<(&Interaction, &Tooltip, &GlobalTransform, &Node)>,
) {
    let (mut visibility, mut style) = r!(container_query.get_mut(tooltip_root.container));
    let mut text = r!(text_query.get_mut(tooltip_root.text));
    let window = r!(window_query.get(window_root.primary));
    let width = window.width();
    let height = window.height();

    for (interaction, tooltip, gt, node) in &interaction_query {
        // Skip nodes that are not hovered.
        if matches!(interaction, Interaction::None) {
            *visibility = Visibility::Hidden;
            continue;
        }

        // Set the tooltip text and make it visible.
        *visibility = Visibility::Inherited;
        text.sections[0].value.clone_from(&tooltip.text);

        // Get the left, right, top, bottom of the target node.
        let rect = node.logical_rect(gt);
        let (left, right, top, bottom) = (
            rect.min.x + tooltip.offset.x,
            rect.max.x + tooltip.offset.x,
            rect.min.y + tooltip.offset.y,
            rect.max.y + tooltip.offset.y,
        );

        // Set the left, right, top, bottom of the tooltip node.
        (style.left, style.right, style.top, style.bottom) = match tooltip.side {
            TooltipSide::Left => (Auto, Px(width - left), Auto, Px(height - bottom)),
            TooltipSide::Right => (Px(right), Auto, Auto, Px(height - bottom)),
            TooltipSide::Top => (Px(left), Auto, Auto, Px(height - top)),
            TooltipSide::Bottom => (Px(left), Auto, Px(bottom), Auto),
        };

        // Exit early (because there's only one tooltip).
        return;
    }
}
