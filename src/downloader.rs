use anyhow::Result;
use lazy_static::lazy_static;
use reqwest::blocking::{get, Response};
use std::time::SystemTime;

const REQUEST_DELAY_MS: u128 = 200;

lazy_static! {
    pub(crate) static ref DOWNLOADER: std::sync::Mutex<DownloadWrapper> =
        std::sync::Mutex::new(DownloadWrapper::new());
}

#[derive(Debug)]
pub(crate) struct DownloadWrapper {
    last_request: SystemTime,
}

impl DownloadWrapper {
    fn new() -> Self {
        Self {
            last_request: SystemTime::now(),
        }
    }

    pub(crate) fn make_request(&mut self, uri: &str) -> Result<Response> {
        while self.last_request.elapsed().unwrap().as_millis() < REQUEST_DELAY_MS {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let result = get(uri)?;
        self.last_request = SystemTime::now();
        Ok(result)
    }
}
