use crate::downloader::DOWNLOADER;
use crate::scryfall_structures::ScryfallCard;

use anyhow::Result;

pub struct Card {
    pub(crate) raw_card: ScryfallCard,
}

impl Card {
    pub fn name(&self) -> &str {
        &self.raw_card.name
    }

    pub fn id(&self) -> &str {
        &self.raw_card.id
    }

    pub fn oracle_text(&self) -> &Option<String> {
        &self.raw_card.oracle_text
    }

    pub fn type_line(&self) -> &Option<String> {
        &self.raw_card.type_line
    }

    pub fn keywords(&self) -> &Vec<String> {
        &self.raw_card.keywords
    }

    pub fn artist(&self) -> &Option<String> {
        &self.raw_card.artist
    }

    pub fn flavor_name(&self) -> &Option<String> {
        &self.raw_card.flavor_name
    }

    pub fn flavor_text(&self) -> &Option<String> {
        &self.raw_card.flavor_text
    }

    pub fn set_name(&self) -> &Option<String> {
        &self.raw_card.flavor_text
    }

    pub fn get_images(&self) -> Result<Vec<Vec<u8>>> {
        if let Some(uris) = &self.raw_card.image_uris {
            Ok(vec![DOWNLOADER
                .lock()
                .unwrap()
                .make_request(&uris.normal)?
                .bytes()?
                .to_vec()])
        } else if let Some(faces) = &self.raw_card.card_faces {
            Ok(faces
                .iter()
                .filter_map(|f| {
                    DOWNLOADER
                        .lock()
                        .unwrap()
                        .make_request(f.image_uris.as_ref().unwrap().normal.as_str())
                        .ok()
                })
                .filter_map(|r| r.bytes().ok())
                .map(|b| b.to_vec())
                .collect())
        } else {
            unreachable!("No images???")
        }
    }
}
