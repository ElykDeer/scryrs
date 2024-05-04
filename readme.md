# scry.rs

A simple alternative to [scryfall](https://github.com/mendess/scryfall-rs) with caching and mass image downloading capabilities.

```rust
use scryers::{
  download_all_cards,
  bulk::{BulkDownload, BulkDownloadType},
};

fn main() {
  // Downloads images for all cards
  scryers::download_all_cards();

  // Allows you to easily iterate over card information, without needing to re-download Scryfall database information (>140Mb!) every time.
  // Lazy loads card info, so initialization speed should be super quick
  let cards = BulkDownload::new("./scryfall.db", BulkDownloadType::UniqueArtwork).unwrap();
  for card in cards.cards() {
    println!("Image at images/{}-0.jpg", card.name());
  }
}
```