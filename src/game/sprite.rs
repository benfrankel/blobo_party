use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {}

struct SpriteAnimationFrame {
    index: usize,
    steps: usize,
}

struct SpriteAnimation {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    frames: Vec<SpriteAnimationFrame>,
}
