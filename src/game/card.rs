use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_kira_audio::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::game::actor::faction::Faction;
use crate::game::card::action::CardAction;
use crate::game::card::action::CardActionKey;
use crate::game::card::action::CardActionMap;
use crate::game::card::action::CardActionModifier;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub mod action;
pub mod attack;
pub mod deck;
pub mod movement;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<CardConfig>, OnPlayCard)>();

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
            if !card.play_sfx_path.is_empty() {
                card.play_sfx = Some(asset_server.load(&card.play_sfx_path));
            }
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
            && self.card_map.values().all(|x| {
                !x.play_sfx
                    .as_ref()
                    .is_some_and(|x| !asset_server.is_loaded_with_dependencies(x))
            })
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
    active: Option<bool>,
}

impl EntityCommand for CardBackground {
    fn apply(self, id: Entity, world: &mut World) {
        let atlas_off = TextureAtlas {
            layout: self.texture_atlas_layout.clone(),
            index: 0,
        };
        let atlas_on = TextureAtlas {
            layout: self.texture_atlas_layout,
            index: 1,
        };
        let atlas = if matches!(self.active, Some(true)) {
            &atlas_on
        } else {
            &atlas_off
        }
        .clone();

        world.entity_mut(id).insert((
            Name::new("CardBackground"),
            ImageBundle {
                image: UiImage::new(self.texture),
                ..default()
            },
            Outline {
                width: Vw(0.4),
                ..default()
            },
            ThemeColor::CardBorder.target::<Outline>(),
            atlas,
        ));

        if self.active.is_none() {
            world.entity_mut(id).insert((
                Interaction::default(),
                InteractionTable {
                    normal: atlas_off.clone(),
                    hovered: atlas_on.clone(),
                    pressed: atlas_on,
                    disabled: atlas_off,
                },
                InteractionSfx,
            ));
        }
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
    /// The earliest level this card will be offered in the level up menu.
    #[serde(default)]
    pub min_level: usize,
    /// The latest level this card will be offered in the level up menu.
    #[serde(default = "inf")]
    pub max_level: usize,
    /// The relative probability of this card being offered in the level up menu.
    #[serde(default = "one")]
    pub weight: f64,

    #[serde(rename = "play_sfx", default)]
    play_sfx_path: String,
    #[serde(skip)]
    pub play_sfx: Option<Handle<AudioSource>>,
    #[serde(default = "one")]
    pub play_sfx_volume: f64,
    #[serde(rename = "action")]
    pub action_key: CardActionKey,
    #[serde(skip)]
    pub action: CardAction,
    pub action_modifier: CardActionModifier,
}

fn one() -> f64 {
    1.0
}

fn inf() -> usize {
    usize::MAX
}

pub fn card(key: impl Into<String>, active: Option<bool>) -> impl EntityCommand {
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
        let border_width = height / 18.0;
        let tooltip_text = format!("{}\n\n{}", card.name, card.description);

        world
            .entity_mut(entity)
            .insert((
                Name::new(name),
                NodeBundle {
                    style: Style {
                        height,
                        border: UiRect::all(border_width),
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

/// An observable event triggered when a card is played.
#[derive(Event)]
pub struct OnPlayCard(pub String);

impl Configure for OnPlayCard {
    fn configure(app: &mut App) {
        app.observe(play_card);
    }
}

fn play_card(
    trigger: Trigger<OnPlayCard>,
    mut commands: Commands,
    config: ConfigRef<CardConfig>,
    audio: Res<Audio>,
    faction_query: Query<&Faction>,
) {
    let entity = r!(trigger.get_entity());
    let config = r!(config.get());
    let card = r!(config.card_map.get(&trigger.event().0));
    let faction = r!(faction_query.get(entity));

    if let (Faction::Player, Some(play_sfx)) = (faction, card.play_sfx.clone()) {
        audio.play(play_sfx).with_volume(card.play_sfx_volume);
    }

    commands.run_system_with_input(card.action.0, (entity, card.action_modifier.clone()));
}
