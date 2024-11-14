use lingua::Language;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WordList {
    pub language: Option<Language>,
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CorrectionList {
    pub language: Option<Language>,
    pub words_and_correction: Vec<(String, String)>,
}
