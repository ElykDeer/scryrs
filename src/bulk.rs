use crate::card::Card;
use crate::downloader::DOWNLOADER;
use crate::scryfall_structures::{ScryfallBulkData, ScryfallResult};

use anyhow::Result;
use serde_json;
use serde_json::Deserializer;
use std::{
    fs::{metadata, File},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    time::{Duration, SystemTime},
};

#[allow(dead_code)]
pub enum BulkDownloadType {
    OracleCards,
    UniqueArtwork,
    DefaultCards,
    AllCards,
    Rulings,
}

impl BulkDownloadType {
    fn to_str(&self) -> &str {
        match self {
            BulkDownloadType::OracleCards => "oracle_cards",
            BulkDownloadType::UniqueArtwork => "unique_artwork",
            BulkDownloadType::DefaultCards => "default_cards",
            BulkDownloadType::AllCards => "all_cards",
            BulkDownloadType::Rulings => "rulings",
        }
    }
}

pub struct BulkDownload {
    card_cache: Vec<Card>,
}

impl BulkDownload {
    pub fn new<P: AsRef<Path>>(path: P, download_type: BulkDownloadType) -> Result<Self> {
        let metadata = metadata(&path)?;

        if !path.as_ref().is_file()
            || if let Ok(modified) = metadata.modified() {
                SystemTime::now().duration_since(modified)? > Duration::from_secs(24 * 60 * 60)
            } else {
                false
            }
        {
            let mut downloader = DOWNLOADER.lock().unwrap();
            if let ScryfallResult::List(list) = downloader
                .make_request("https://api.scryfall.com/bulk-data")?
                .json()?
            {
                let bulk = list
                    .data
                    .into_iter()
                    .map(|e| e.into())
                    .find(|b: &ScryfallBulkData| b.t == download_type.to_str())
                    .unwrap();

                eprint!("Downloading cards...");
                downloader
                    .make_request(&bulk.download_uri)?
                    .copy_to(&mut File::create(&path).unwrap())
                    .unwrap();
                println!("done!");

                // Trim file
                eprint!("Triming file...");
                let input_file = File::open(&path)?;
                let output_file = File::create("temp")?;
                let mut input_reader = BufReader::new(input_file);
                let mut output_writer = BufWriter::new(output_file);
                let mut line = String::new();
                while input_reader.read_line(&mut line)? > 0 {
                    let line_length = line.trim_end().len();
                    if line_length > 4 {
                        if &line[line_length - 1..] == ",\n" {
                            write!(output_writer, "{}", &line[..line_length - 1])?;
                        } else {
                            write!(output_writer, "{}", &line)?;
                        }
                    }
                    line.clear();
                }
                std::fs::rename("temp", &path)?;
                println!("done!");
            } else {
                anyhow::bail!("Could not fetch bulk data");
            }
        }

        Ok(Self {
            card_cache: Deserializer::from_reader(BufReader::new(
                File::open(path.as_ref().to_string_lossy().to_string()).unwrap(),
            ))
            .into_iter()
            .map(|card| Card {
                raw_card: card.unwrap(),
            })
            .collect(),
        })
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.card_cache.as_ref()
    }

    pub fn get_card_by_id(&self, id: &str) -> Result<&Card> {
        for card in &self.card_cache {
            if card.id() == id {
                return Ok(card);
            }
        }
        anyhow::bail!("Could not find card of that id");
    }
}
