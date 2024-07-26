use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use serde::Deserialize;
use serde::Serialize;

use crate::game::card::action::CardAction;
use crate::game::card::action::CardActionConfig;
use crate::game::card::action::CardActionKey;
use crate::game::card::action::CardActionMap;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub mod action;
pub mod attack;
pub mod deck;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<CardConfig>, AddCardEvent)>();

    app.add_plugins((
        action::plugin,
        attack::plugin,
        deck::plugin,
        movement::plugin,
    ));
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct CardConfig {
    // Deck:
    pub deck_cap: usize,

    // Cards:
    pub card_height: Val,
    pub card_background_map: HashMap<String, CardBackground>,
    pub card_icon_map: HashMap<String, CardIcon>,
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

        for icon in self.card_icon_map.values_mut() {
            icon.texture = asset_server.load(&icon.texture_path);
        }

        for card in self.card_map.values_mut() {
            card.action = *c!(card_action_map.0.get(&card.action_key));
        }
    }

    fn is_ready(&self, asset_server: &AssetServer) -> bool {
        self.card_background_map
            .values()
            .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
            && self
                .card_icon_map
                .values()
                .all(|x| asset_server.is_loaded_with_dependencies(&x.texture))
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize, Clone)]
pub struct CardBackground {
    #[serde(rename = "texture")]
    texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
    texture_atlas_grid: TextureAtlasGrid,
    #[serde(skip)]
    pub texture_atlas_layout: Handle<TextureAtlasLayout>,
    #[serde(skip)]
    active: bool,
}

impl EntityCommand for CardBackground {
    fn apply(self, id: Entity, world: &mut World) {
        world.entity_mut(id).insert((
            Name::new("CardBackground"),
            ImageBundle {
                image: UiImage::new(self.texture),
                ..default()
            },
            TextureAtlas {
                layout: self.texture_atlas_layout,
                index: if self.active { 1 } else { 0 },
            },
        ));
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize, Clone)]
pub struct CardIcon {
    #[serde(rename = "texture")]
    texture_path: String,
    #[serde(skip)]
    pub texture: Handle<Image>,
}

impl EntityCommand for CardIcon {
    fn apply(self, id: Entity, world: &mut World) {
        world.entity_mut(id).insert((
            Name::new("CardIcon"),
            ImageBundle {
                image: UiImage::new(self.texture),
                ..default()
            },
            ThemeColor::CardBorder.target::<UiImage>(),
        ));
    }
}

#[derive(Reflect, Serialize, Deserialize, Clone)]
pub struct Card {
    pub name: String,
    pub description: String,
    #[serde(rename = "background")]
    pub background_key: String,
    #[serde(rename = "icon")]
    pub icon_key: String,
    #[serde(rename = "action", default)]
    action_key: CardActionKey,
    #[serde(skip)]
    pub action: CardAction,
    pub action_config: CardActionConfig, // TODO: Naming
}

fn card(key: impl Into<String>, active: bool) -> impl EntityCommand {
    let key = key.into();

    move |entity: Entity, world: &mut World| {
        let config = SystemState::<ConfigRef<CardConfig>>::new(world).get(world);
        let config = r!(config.get());
        let card = r!(config.card_map.get(&key));
        let mut background = r!(config.card_background_map.get(&card.background_key)).clone();
        background.active = active;
        let icon = r!(config.card_icon_map.get(&card.icon_key)).clone();
        let name = format!("Card(\"{}\")", card.name);
        let height = config.card_height;
        let tooltip_text = format!("{}\n\n{}", card.name, card.description);

        world
            .entity_mut(entity)
            .insert((
                Name::new(name),
                NodeBundle {
                    style: Style {
                        height,
                        border: UiRect::all(Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                ThemeColor::CardBorder.target::<BorderColor>(),
                Interaction::default(),
                Tooltip {
                    text: tooltip_text,
                    self_anchor: Anchor::TopCenter,
                    tooltip_anchor: Anchor::BottomCenter,
                    offset: Vec2::ZERO,
                },
            ))
            .with_children(|children| {
                children.spawn_with(background).with_children(|children| {
                    children.spawn_with(icon);
                });
            });
    }
}

#[derive(Event)]
pub struct AddCardEvent(pub String);

impl Configure for AddCardEvent {
    fn configure(app: &mut App) {
        app.add_event::<Self>();
    }
}
