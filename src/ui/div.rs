use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;

use crate::ui::prelude::*;

pub trait NodeExtDiv {
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

impl NodeExtDiv for Node {
    const FILL: Self = {
        let mut x = Self::DEFAULT;
        x.width = Percent(100.0);
        x.height = Percent(100.0);
        x
    };

    const ROW: Self = Self::FILL;

    const COLUMN: Self = {
        let mut x = Self::FILL;
        x.flex_direction = FlexDirection::Column;
        x
    };

    const ROW_TOP: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Start;
        x
    };

    const ROW_MID: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Center;
        x
    };

    const ROW_BOTTOM: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::End;
        x
    };

    const ROW_CENTER: Self = {
        let mut x = Self::ROW;
        x.align_items = AlignItems::Center;
        x.justify_content = JustifyContent::Center;
        x
    };

    const COLUMN_LEFT: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Start;
        x
    };

    const COLUMN_MID: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Center;
        x
    };

    const COLUMN_RIGHT: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::End;
        x
    };

    const COLUMN_CENTER: Self = {
        let mut x = Self::COLUMN;
        x.align_items = AlignItems::Center;
        x.justify_content = JustifyContent::Center;
        x
    };

    const ABS_FILL: Self = {
        let mut x = Self::FILL;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_ROW: Self = {
        let mut x = Self::ROW;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_COLUMN: Self = {
        let mut x = Self::COLUMN;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_ROW_TOP: Self = {
        let mut x = Self::ROW_TOP;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_ROW_MID: Self = {
        let mut x = Self::ROW_MID;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_ROW_BOTTOM: Self = {
        let mut x = Self::ROW_BOTTOM;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_ROW_CENTER: Self = {
        let mut x = Self::ROW_CENTER;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_COLUMN_LEFT: Self = {
        let mut x = Self::COLUMN_LEFT;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_COLUMN_MID: Self = {
        let mut x = Self::COLUMN_MID;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_COLUMN_RIGHT: Self = {
        let mut x = Self::COLUMN_RIGHT;
        x.position_type = PositionType::Absolute;
        x
    };

    const ABS_COLUMN_CENTER: Self = {
        let mut x = Self::COLUMN_CENTER;
        x.position_type = PositionType::Absolute;
        x
    };

    fn div(self) -> impl EntityCommand<World> {
        move |mut entity: EntityWorldMut| {
            entity.insert(self);
        }
    }
}
