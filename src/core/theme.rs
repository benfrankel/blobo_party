use std::marker::PhantomData;
use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumCount;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Default to CSS loading screen color.
    app.insert_resource(ClearColor(Color::srgb(0.161, 0.157, 0.231)));

    app.configure::<(
        ConfigHandle<ThemeConfig>,
        ThemeColorFor<Sprite>,
        ThemeColorFor<ImageNode>,
        ThemeColorFor<BackgroundColor>,
        ThemeColorFor<BorderColor>,
        ThemeColorFor<Outline>,
        ThemeColorFor<TextColor>,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub colors: ThemeColorList,
    // TODO: pub fonts: ThemeFontList,
}

impl Config for ThemeConfig {
    const PATH: &'static str = "config/theme.ron";
    const EXTENSION: &'static str = "theme.ron";

    fn on_load(&mut self, world: &mut World) {
        world.resource_mut::<ClearColor>().0 = self.colors[ThemeColor::Body];
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeColorList([Color; ThemeColor::COUNT]);

impl Index<ThemeColor> for ThemeColorList {
    type Output = Color;

    fn index(&self, index: ThemeColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

/// See: <https://getbootstrap.com/docs/5.3/customize/color/>
#[derive(Reflect, Clone, Copy, Default, EnumCount)]
pub enum ThemeColor {
    // Absolute colors
    #[default]
    White,
    Invisible,

    // Semantic colors
    Body,
    BodyText,

    Primary,
    PrimaryHovered,
    PrimaryPressed,
    PrimaryDisabled,
    PrimaryText,

    // Misc UI colors
    Popup,
    Indicator,
    CardBorder,
    Overlay,
}

impl ThemeColor {
    pub const fn target<C: Component + ColorMut>(self) -> ThemeColorFor<C> {
        ThemeColorFor(self, PhantomData)
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ThemeColorFor<C: Component + ColorMut>(
    pub ThemeColor,
    #[reflect(ignore)] PhantomData<C>,
);

impl<C: Component + ColorMut + TypePath> Configure for ThemeColorFor<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_theme_color_for::<C>.in_set(UpdateSet::SyncLate),
        );
    }
}

fn apply_theme_color_for<C: Component + ColorMut>(
    theme: ConfigRef<ThemeConfig>,
    mut color_query: Query<(&ThemeColorFor<C>, &mut C)>,
) {
    let palette = &r!(theme.get()).colors;
    for (theme_color, mut color) in &mut color_query {
        *color.color_mut() = palette[theme_color.0];
    }
}

pub trait ColorMut {
    fn color_mut(&mut self) -> &mut Color;
}

impl ColorMut for Sprite {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl ColorMut for ImageNode {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl ColorMut for BackgroundColor {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.0
    }
}

impl ColorMut for BorderColor {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.0
    }
}

impl ColorMut for Outline {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl ColorMut for TextColor {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.0
    }
}
