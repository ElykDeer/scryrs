use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(tag = "object")]
pub enum ScryfallResult {
    #[serde(rename = "error")]
    Error(ScryfallError),
    #[serde(rename = "list")]
    List(ScryfallList),
    #[serde(rename = "bulk_data")]
    BulkData(ScryfallBulkData),
    #[serde(rename = "card")]
    Card(ScryfallCard),
}

impl From<ScryfallResult> for ScryfallBulkData {
    fn from(value: ScryfallResult) -> ScryfallBulkData {
        if let ScryfallResult::BulkData(result) = value {
            result
        } else {
            unreachable!("Well that wasn't supposed to happen...");
        }
    }
}

impl From<ScryfallResult> for ScryfallCard {
    fn from(value: ScryfallResult) -> ScryfallCard {
        if let ScryfallResult::Card(result) = value {
            result
        } else {
            unreachable!("Well that wasn't supposed to happen...");
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallError {
    pub status: i64,
    pub code: String,
    pub details: String,
    #[serde(rename = "type")]
    pub t: Option<String>,
    pub warnings: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallList {
    pub data: Vec<ScryfallResult>,
    pub has_more: bool,
    pub next_page: Option<String>,
    pub total_cards: Option<i64>,
    pub warnings: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallBulkData {
    pub id: String,
    pub uri: String,
    #[serde(rename = "type")]
    pub t: String,
    pub name: String,
    pub description: String,
    pub download_uri: String,
    pub updated_at: String,
    pub size: i64,
    pub content_type: String,
    pub content_encoding: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallRelatedCard {
    pub object: String,
    pub id: String,
    pub component: String,
    pub name: String,
    pub type_line: String,
    pub uri: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallCardImages {
    pub png: String,
    pub border_crop: String,
    pub art_crop: String,
    pub large: String,
    pub normal: String,
    pub small: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallCardFace {
    pub artist: Option<String>,
    pub cmc: Option<f64>,
    pub color_indicator: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<String>,
    pub image_uris: Option<ScryfallCardImages>,
    pub layout: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub object: String,
    pub oracle_id: Option<String>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallCard {
    pub arena_id: Option<i64>,
    pub id: String,
    pub lang: String,
    pub mtgo_id: Option<i64>,
    pub mtgo_foil_id: Option<i64>,
    pub multiverse_ids: Option<Vec<i64>>,
    pub tcgplayer_id: Option<i64>,
    pub tcgplayer_etched_id: Option<i64>,
    pub cardmarket_id: Option<i64>,
    pub oracle_id: Option<String>,
    pub prints_search_uri: String,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub uri: String,
    pub all_parts: Option<Vec<ScryfallRelatedCard>>,
    pub card_faces: Option<Vec<ScryfallCardFace>>,
    pub cmc: Option<f64>,
    pub color_identity: Vec<String>,
    pub color_indicator: Option<Vec<String>>,
    pub colors: Option<Vec<String>>,
    pub edhrec_rank: Option<i64>,
    pub hand_modifier: Option<String>,
    pub keywords: Vec<String>,
    pub layout: String,
    pub legalities: HashMap<String, Option<String>>,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub penny_rank: Option<i64>,
    pub power: Option<String>,
    pub produced_mana: Option<Vec<String>>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub artist: Option<String>,
    pub attraction_lights: Option<Vec<i64>>,
    pub booster: bool,
    pub border_color: String,
    pub card_back_id: Option<String>,
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<String>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<String>>,
    pub frame: String,
    pub full_art: bool,
    pub games: Vec<String>,
    pub highres_image: bool,
    pub illustration_id: Option<String>,
    pub image_status: String,
    pub image_uris: Option<ScryfallCardImages>,
    pub prices: HashMap<String, Option<String>>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: Option<HashMap<String, Option<String>>>,
    pub rarity: String,
    pub related_uris: HashMap<String, Option<String>>,
    pub released_at: String,
    pub reprint: bool,
    pub scryfall_set_uri: String,
    pub set_name: String,
    pub set_search_uri: String,
    pub set_type: String,
    pub set_uri: String,
    pub set: String,
    pub set_id: String,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<String>,
    pub security_stamp: Option<String>,
    pub watermark: Option<String>,
    #[serde(rename = "preview.previewed_at")]
    pub previewed_at: Option<String>,
    #[serde(rename = "preview.source_uri")]
    pub source_uri: Option<String>,
    #[serde(rename = "preview.source")]
    pub source: Option<String>,
}
