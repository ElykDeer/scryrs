pub mod bulk;
pub mod card;
pub mod downloader;
pub mod scryfall_structures;

use bulk::{BulkDownload, BulkDownloadType};
use std::{fs, io::Write, time::SystemTime};

/// Downloads high resolution copies of all the cards. This over-respects Scryfall's api limits. Takes about 3 hours to complete.
pub fn download_all_cards() {
    let mut scryer = BulkDownload::new("./scryfall.db", BulkDownloadType::UniqueArtwork).unwrap();
    let mut card_index = 1;

    let start_time = SystemTime::now();
    let cards = scryer.cards();
    let total_cards = cards.len();
    for card in cards {
        println!(
            "{:0>5}/{} ({:.2}m..est {:.2}m remaining) : Downloading {}",
            card_index,
            total_cards,
            start_time.elapsed().unwrap().as_secs_f64() / 60.0,
            (card_index as f64 / start_time.elapsed().unwrap().as_secs_f64()
                * (total_cards - card_index) as f64)
                / 60.0,
            card.name()
        );
        for (card_art_n, image) in (card.get_images().unwrap()).into_iter().enumerate() {
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("images/{}-{}.jpg", card.id(), card_art_n))
                .unwrap();
            file.write_all(&image).unwrap();
        }
        card_index += 1;
    }
}

/// If any of the card images fail, you can use this to download the images for a specific ID again.
pub fn download_cards_for_id(id: &str) {
    let mut scryer = BulkDownload::new("./scryfall.db", BulkDownloadType::UniqueArtwork).unwrap();
    for (card_art_n, image) in (scryer.get_card_by_id(id).unwrap().get_images().unwrap())
        .into_iter()
        .enumerate()
    {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("images/{}-{}.jpg", id, card_art_n))
            .unwrap();
        file.write_all(&image).unwrap();
    }
}
