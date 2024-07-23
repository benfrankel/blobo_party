use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::theme::ThemeColor;
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
    pub fn layer(&self) -> LayerMask {
        match self {
            Self::Player => GameLayer::Player,
            Self::Enemy => GameLayer::Enemy,
        }
        .into()
    }

    pub fn projectile_color(&self) -> ThemeColor {
        match self {
            Self::Player => ThemeColor::PlayerProjectile,
            Self::Enemy => ThemeColor::EnemyProjectile,
        }
    }
}
