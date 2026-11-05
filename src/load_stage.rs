//! Current Android load stage, keyed by address so concurrent tabs stay separate.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy)]
pub enum Stage {
    NetworkFetching,
    RegularTextExtraction,
    OcrTextExtraction,
}

impl Stage {
    pub fn label(self) -> &'static str {
        match self {
            Self::NetworkFetching => "Network fetching",
            Self::RegularTextExtraction => "Regular text extraction",
            Self::OcrTextExtraction => "OCR text extraction",
        }
    }
}

static STAGES: OnceLock<Mutex<HashMap<String, Stage>>> = OnceLock::new();

fn key(address: &str) -> &str {
    address
        .split_once('#')
        .map_or(address, |(address, _)| address)
}

fn stages() -> &'static Mutex<HashMap<String, Stage>> {
    STAGES.get_or_init(|| Mutex::new(HashMap::new()))
}

pub struct Load(String);

pub fn begin(address: &str) -> Load {
    stages()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .insert(key(address).to_owned(), Stage::NetworkFetching);
    Load(key(address).to_owned())
}

pub fn set(address: &str, stage: Stage) {
    let mut stages = stages()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(current) = stages.get_mut(key(address)) {
        *current = stage;
    }
}

pub fn current(address: &str) -> &'static str {
    stages()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .get(key(address))
        .copied()
        .unwrap_or(Stage::NetworkFetching)
        .label()
}

impl Drop for Load {
    fn drop(&mut self) {
        stages()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .remove(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_stage_until_load_finishes() {
        let load = begin("https://example.com/a.pdf");
        assert_eq!(current("https://example.com/a.pdf"), "Network fetching");
        set("https://example.com/a.pdf", Stage::OcrTextExtraction);
        assert_eq!(current("https://example.com/a.pdf"), "OCR text extraction");
        set(
            "https://example.com/a.pdf#definition",
            Stage::RegularTextExtraction,
        );
        assert_eq!(
            current("https://example.com/a.pdf#definition"),
            "Regular text extraction"
        );
        drop(load);
        assert_eq!(current("https://example.com/a.pdf"), "Network fetching");
    }
}
