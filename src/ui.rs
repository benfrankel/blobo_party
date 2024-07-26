//! Custom UI tools.

#![allow(dead_code, unused_imports)]

pub mod div;
pub mod font;
pub mod interaction;
pub mod tooltip;
pub mod widget;

pub mod prelude {
    pub use bevy::ui::Val::*;

    pub use super::div::StyleExtDiv as _;
    pub use super::font::parse_rich;
    pub use super::font::parse_rich_custom;
    pub use super::font::DynamicFontSize;
    pub use super::font::BOLD_FONT_HANDLE;
    pub use super::font::FONT_HANDLE;
    pub use super::font::THICK_FONT_HANDLE;
    pub use super::interaction::InteractionTable;
    pub use super::interaction::IsDisabled;
    pub use super::widget;
    pub use super::UiRoot;
    pub use crate::core::theme::ThemeColor;
    pub use crate::core::theme::ThemeColorFor;
    pub use crate::core::theme::ThemeColorForText;
}

use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_mod_picking::prelude::*;
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
                .spawn((
                    Name::new("Ui"),
                    NodeBundle {
                        style: Style::COLUMN_MID,
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .id(),
        }
    }
}

fn clear_ui_root(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}
