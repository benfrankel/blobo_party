use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;
use bevy_kira_audio::prelude::*;
use iyes_progress::prelude::*;
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

    fn count_progress(&self, asset_server: &AssetServer) -> Progress {
        let mut progress = true.into();

        for card_bg in self.card_background_map.values() {
            progress += asset_server
                .is_loaded_with_dependencies(&card_bg.texture)
                .into();
        }
        for card_icon in self.card_icon_map.values() {
            progress += asset_server
                .is_loaded_with_dependencies(&card_icon.texture)
                .into();
        }
        for card in self.card_map.values() {
            progress += card
                .play_sfx
                .as_ref()
                .is_none_or(|x| asset_server.is_loaded_with_dependencies(x))
                .into();
        }

        progress
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
        let image_off = ImageNode::from_atlas_image(
            self.texture.clone(),
            TextureAtlas {
                layout: self.texture_atlas_layout.clone(),
                index: 0,
            },
        );
        let image_on = ImageNode::from_atlas_image(
            self.texture,
            TextureAtlas {
                layout: self.texture_atlas_layout,
                index: 1,
            },
        );
        let image = if matches!(self.active, Some(true)) {
            &image_on
        } else {
            &image_off
        }
        .clone();

        world.entity_mut(id).insert((
            Name::new("CardBackground"),
            image,
            Outline {
                width: Vw(0.4),
                ..default()
            },
            ThemeColor::CardBorder.target::<Outline>(),
        ));

        if self.active.is_none() {
            world.entity_mut(id).insert((
                Interaction::default(),
                InteractionTable {
                    normal: image_off.clone(),
                    hovered: image_on.clone(),
                    pressed: image_on,
                    disabled: image_off,
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
            ImageNode::from(self.texture),
            ThemeColor::CardBorder.target::<ImageNode>(),
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

fn inf() -> usize {
    usize::MAX
}

fn one() -> f64 {
    1.0
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
        let tooltip_text = format!("[b]{}[r]\n\n{}", card.name, card.description);
        let mut tooltip_text = parse_rich(tooltip_text);
        // TODO: Workaround for `DynamicFontSize` not being inherited by `TextSpan`
        //       child entities of the primary tooltip text entity.
        for section in &mut tooltip_text {
            section.style.font_size = 16.0;
        }

        world
            .entity_mut(entity)
            .insert((
                Name::new(name),
                Node {
                    height,
                    border: UiRect::all(border_width),
                    ..default()
                },
                ThemeColor::CardBorder.target::<BorderColor>(),
                Interaction::default(),
                Tooltip::fixed(Anchor::TopCenter, tooltip_text).with_justify(JustifyText::Center),
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
        app.add_observer(play_card);
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
