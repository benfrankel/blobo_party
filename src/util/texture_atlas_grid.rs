use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct TextureAtlasGrid {
    tile_size: UVec2,
    columns: u32,
    rows: u32,
    padding: Option<UVec2>,
    offset: Option<UVec2>,
}

impl From<&TextureAtlasGrid> for TextureAtlasLayout {
    fn from(value: &TextureAtlasGrid) -> Self {
        Self::from_grid(
            value.tile_size,
            value.columns,
            value.rows,
            value.padding,
            value.offset,
        )
    }
}
