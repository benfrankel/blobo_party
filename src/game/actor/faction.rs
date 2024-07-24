use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::GameLayer;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<Faction>();
}

#[derive(Component, Reflect, Copy, Clone)]
#[reflect(Component)]
pub enum Faction {
    Player,
    Enemy,
}

impl Configure for Faction {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

impl Faction {
    #[allow(dead_code)]
    pub fn is_player(&self) -> bool {
        matches!(self, Self::Player)
    }

    pub fn is_enemy(&self) -> bool {
        matches!(self, Self::Enemy)
    }

    pub fn layer(&self) -> LayerMask {
        match self {
            Self::Player => GameLayer::Player,
            Self::Enemy => GameLayer::Enemy,
        }
        .into()
    }
}
