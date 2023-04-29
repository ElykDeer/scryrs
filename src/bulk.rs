use crate::card::Card;
use crate::downloader::DOWNLOADER;
use crate::scryfall_structures::{ScryfallBulkData, ScryfallCard, ScryfallResult};

use anyhow::Result;
use serde_json;
use serde_json::Deserializer;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
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
    file: String,
}

impl BulkDownload {
    pub fn new<P: AsRef<Path>>(path: P, download_type: BulkDownloadType) -> Result<Self> {
        if !path.as_ref().is_file() {
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
                        println!("`{:?}`", &line[line_length - 1..]);
                        if &line[line_length - 1..] == ",\n" {
                            write!(output_writer, "{}", &line[2..line_length - 1])?;
                        } else {
                            write!(output_writer, "{}", &line[2..])?;
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
            file: path.as_ref().to_string_lossy().to_string(),
        })
    }

    pub fn cards(&self) -> impl Iterator<Item = Card> + '_ {
        Deserializer::from_reader(BufReader::new(File::open(&self.file).unwrap()))
            .into_iter::<ScryfallCard>()
            .map(|raw_card| Card {
                raw_card: raw_card.unwrap(),
            })
    }
}
