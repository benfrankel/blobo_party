use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::core::UpdateSet;
use crate::game::step::on_step;
use crate::game::step::Step;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<SpriteAnimation>();
}

#[derive(Reflect, Serialize, Deserialize, Copy, Clone)]
pub struct SpriteAnimationFrame {
    pub index: usize,
    pub steps: usize,
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Component)]
pub struct SpriteAnimation {
    pub frames: Vec<SpriteAnimationFrame>,
    #[serde(skip)]
    pub total_steps: usize,
}

impl Configure for SpriteAnimation {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_sprite_animation
                .in_set(UpdateSet::Update)
                .run_if(on_step(1)),
        );
    }
}

impl SpriteAnimation {
    // TODO: Does serde support syncing a value after deserialization?
    pub fn calculate_total_steps(&mut self) {
        self.total_steps = self.frames.iter().map(|x| x.steps).sum();
    }

    /// Calculate the texture atlas index of the animation after `steps` steps.
    fn index(&self, steps: usize) -> usize {
        let mut steps = steps % self.total_steps;
        let mut i = 0;
        while steps >= self.frames[i].steps {
            steps -= self.frames[i].steps;
            i += 1;
        }

        self.frames[i].index
    }
}

fn update_sprite_animation(
    step: Res<Step>,
    mut anim_query: Query<(&SpriteAnimation, &mut TextureAtlas)>,
) {
    for (anim, mut atlas) in &mut anim_query {
        atlas.index = anim.index(step.total);
    }
}
