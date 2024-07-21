use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Reflect, Serialize, Deserialize)]
pub struct TextureAtlasGrid {
    tile_size: UVec2,
    columns: u32,
    rows: u32,
    padding: Option<UVec2>,
    offset: Option<UVec2>,
}

impl Into<TextureAtlasLayout> for &TextureAtlasGrid {
    fn into(self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            self.tile_size,
            self.columns,
            self.rows,
            self.padding,
            self.offset,
        )
    }
}
