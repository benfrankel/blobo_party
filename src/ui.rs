//! Custom UI tools.

#![allow(dead_code)]

pub mod div;
pub mod font;
pub mod interaction;
pub mod tooltip;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use bevy::ui::Val::*;
    pub use pyri_tooltip::prelude::*;

    pub use super::UiRoot;
    pub use super::div::NodeExtDiv as _;
    pub use super::font::BOLD_FONT_HANDLE;
    pub use super::font::DynamicFontSize;
    pub use super::font::FONT_HANDLE;
    pub use super::font::THICK_FONT_HANDLE;
    pub use super::font::parse_rich;
    pub use super::font::parse_rich_custom;
    pub use super::interaction::InteractionSfx;
    pub use super::interaction::InteractionTable;
    pub use super::interaction::IsDisabled;
    pub use super::widget;
    pub use crate::core::theme::ThemeColor;
}

use bevy::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<UiRoot>();

    app.add_plugins((font::plugin, interaction::plugin, tooltip::plugin));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct UiRoot {
    pub body: Entity,
}

impl Configure for UiRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(StateFlush, Screen::ANY.on_exit(clear_ui_root));
    }
}

impl FromWorld for UiRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            body: world
                .spawn((Name::new("Ui"), Node::COLUMN_MID, PickingBehavior::IGNORE))
                .id(),
        }
    }
}

fn clear_ui_root(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}
