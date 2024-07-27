use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::audio::music::on_beat;
use crate::game::audio::music::Beat;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<SpriteAnimation>();
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone)]
pub struct SpriteAnimationFrame {
    pub index: usize,
    pub beats: usize,
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Component)]
pub struct SpriteAnimation {
    pub frames: Vec<SpriteAnimationFrame>,
    #[serde(skip)]
    pub total_beats: usize,
}

impl Configure for SpriteAnimation {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_sprite_animation
                .in_set(UpdateSet::Update)
                .run_if(on_beat(1)),
        );
    }
}

impl SpriteAnimation {
    // TODO: Does serde support syncing a value after deserialization?
    pub fn calculate_total_beats(&mut self) {
        self.total_beats = self.frames.iter().map(|x| x.beats).sum();
    }

    /// Calculate the texture atlas index of the animation after `beats` beats.
    fn index(&self, beats: usize) -> usize {
        let mut beats = beats % self.total_beats;
        let mut i = 0;
        while beats >= self.frames[i].beats {
            beats -= self.frames[i].beats;
            i += 1;
        }

        self.frames[i].index
    }
}

fn update_sprite_animation(
    beat: Res<Beat>,
    mut anim_query: Query<(&SpriteAnimation, &mut TextureAtlas)>,
) {
    for (anim, mut atlas) in &mut anim_query {
        atlas.index = anim.index(beat.total);
    }
}
