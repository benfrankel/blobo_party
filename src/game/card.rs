use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::card::action::CardAction;
use crate::game::card::action::CardActionConfig;
use crate::game::card::action::CardActionKey;
use crate::game::card::action::CardActionMap;
use crate::util::prelude::*;

pub mod action;
pub mod attack;
pub mod deck;
pub mod deck_dock;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<CardConfig>, AddCardEvent)>();

    app.add_plugins((
        action::plugin,
        attack::plugin,
        deck::plugin,
        deck_dock::plugin,
        movement::plugin,
    ));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardConfig {
    // Deck:
    pub deck_cap: usize,
    pub deck_height: Val,
    pub deck_column_gap: Val,

    // Cards:
    pub card_background_map: HashMap<String, CardBackground>,
    pub card_map: HashMap<String, Card>,
}

impl Config for CardConfig {
    const PATH: &'static str = "config/card.ron";
    const EXTENSION: &'static str = "card.ron";

    fn on_load(&mut self, world: &mut World) {
        let (asset_server, card_action_map, mut layouts) = SystemState::<(
            Res<AssetServer>,
            Res<CardActionMap>,
            ResMut<Assets<TextureAtlasLayout>>,
        )>::new(world)
        .get_mut(world);

        for background in self.card_background_map.values_mut() {
            background.texture = asset_server.load(&background.texture_path);
            background.texture_atlas_layout = layouts.add(&background.texture_atlas_grid);
        }

        for card in self.card_map.values_mut() {
            card.icon_texture = asset_server.load(&card.icon_texture_path);
            card.action = *c!(card_action_map.0.get(&card.action_key));
        }
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.card_background_map
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
            && self
                .card_map
                .values()
                .all(|x| asset_server.is_loaded_with_dependencies(&x.icon_texture))
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardBackground {
    #[serde(rename = "texture")]
    texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
    texture_atlas_grid: TextureAtlasGrid,
    #[serde(skip)]
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub description: String,
    pub background: String,
    #[serde(rename = "icon_texture")]
    icon_texture_path: String,
    #[serde(skip)]
    pub icon_texture: Handle<Image>,
    #[serde(rename = "action", default)]
    action_key: CardActionKey,
    #[serde(skip)]
    pub action: CardAction,
    pub action_config: CardActionConfig, // TODO: Naming
}

#[derive(Event)]
pub struct AddCardEvent(pub String);

impl Configure for AddCardEvent {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
    }
}
