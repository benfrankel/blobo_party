use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::ui::prelude::*;

pub trait StyleExtDiv {
    const FILL: Self;
    const ROW: Self;
    const COLUMN: Self;

    const ROW_TOP: Self;
    const ROW_MID: Self;
    const ROW_BOTTOM: Self;
    const ROW_CENTER: Self;

    const COLUMN_LEFT: Self;
    const COLUMN_MID: Self;
    const COLUMN_RIGHT: Self;
    const COLUMN_CENTER: Self;

    const ABS_FILL: Self;
    const ABS_ROW: Self;
    const ABS_COLUMN: Self;

    const ABS_ROW_TOP: Self;
    const ABS_ROW_MID: Self;
    const ABS_ROW_BOTTOM: Self;
    const ABS_ROW_CENTER: Self;

    const ABS_COLUMN_LEFT: Self;
    const ABS_COLUMN_MID: Self;
    const ABS_COLUMN_RIGHT: Self;
    const ABS_COLUMN_CENTER: Self;

    fn div(self) -> impl EntityCommand<World>;
}

impl StyleExtDiv for Style {
    const FILL: Self = {
        let mut style = Self::DEFAULT;
        style.width = Percent(100.0);
        style.height = Percent(100.0);
        style
    };

    const ROW: Self = Self::FILL;

    const COLUMN: Self = {
        let mut style = Self::FILL;
        style.flex_direction = FlexDirection::Column;
        style
    };

    const ROW_TOP: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Start;
        style
    };

    const ROW_MID: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Center;
        style
    };

    const ROW_BOTTOM: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::End;
        style
    };

    const ROW_CENTER: Self = {
        let mut style = Style::ROW;
        style.align_items = AlignItems::Center;
        style.justify_content = JustifyContent::Center;
        style
    };

    const COLUMN_LEFT: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Start;
        style
    };

    const COLUMN_MID: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Center;
        style
    };

    const COLUMN_RIGHT: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::End;
        style
    };

    const COLUMN_CENTER: Self = {
        let mut style = Style::COLUMN;
        style.align_items = AlignItems::Center;
        style.justify_content = JustifyContent::Center;
        style
    };

    const ABS_FILL: Self = {
        let mut style = Style::FILL;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_ROW: Self = {
        let mut style = Style::ROW;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_COLUMN: Self = {
        let mut style = Style::COLUMN;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_ROW_TOP: Self = {
        let mut style = Style::ROW_TOP;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_ROW_MID: Self = {
        let mut style = Style::ROW_MID;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_ROW_BOTTOM: Self = {
        let mut style = Style::ROW_BOTTOM;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_ROW_CENTER: Self = {
        let mut style = Style::ROW_CENTER;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_COLUMN_LEFT: Self = {
        let mut style = Style::COLUMN_LEFT;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_COLUMN_MID: Self = {
        let mut style = Style::COLUMN_MID;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_COLUMN_RIGHT: Self = {
        let mut style = Style::COLUMN_RIGHT;
        style.position_type = PositionType::Absolute;
        style
    };

    const ABS_COLUMN_CENTER: Self = {
        let mut style = Style::COLUMN_CENTER;
        style.position_type = PositionType::Absolute;
        style
    };

    fn div(self) -> impl EntityCommand<World> {
        move |mut entity: EntityWorldMut| {
            entity.insert(NodeBundle {
                style: self,
                ..default()
            });
        }
    }
}
