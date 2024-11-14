use lingua::Language;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WordList {
    pub language: Option<Language>,
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CorrectionList {
    // pub language: Option<Language>,
    pub words_and_correction: Vec<(usize, String, String)>,
}

impl CorrectionList {
    pub fn is_empty(&self) -> bool {
        self.words_and_correction.is_empty()
    }
}
