use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use console::Term;
use lingua::{Language, LanguageDetectorBuilder};
use serde::{Deserialize, Serialize};

use crate::LANGUAGES;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WordList {
    #[serde(skip_serializing_if = "Option::is_none")]
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

pub fn build_word_list(term: &mut Term) -> anyhow::Result<WordList> {
    let mut words = Vec::new();
    for i in 1.. {
        let mut buf = String::new();
        print!("input word {i}, or q to quit: ");
        stdout().flush()?;
        stdin().read_line(&mut buf)?;
        let tmp = buf.trim();
        if tmp == "q" || tmp.is_empty() {
            break;
        }
        words.push(buf.trim().to_owned());
    }

    let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
    let detected = words.join(" ");
    let mut language = detector.detect_language_of(detected);

    let mut invalid_flag = false;
    loop {
        let mut buf = String::new();
        if !invalid_flag && language.is_some() {
            print!(
                "language detected: {}, enter to use it, or change to: ",
                language.unwrap()
            );
        } else if invalid_flag {
            print!("invalid language! input one among {LANGUAGES:?}: ");
        } else {
            print!("input to select a language: ");
        }
        stdout().flush()?;
        stdin().read_line(&mut buf)?;
        let tmp = buf.trim();
        if tmp.is_empty() {
            break;
        }
        let lang = Language::from_str(tmp);
        match lang {
            Ok(l) => {
                language = Some(l);
                break;
            }
            Err(_) => {
                invalid_flag = true;
                term.move_cursor_up(1)?;
                term.clear_line()?;
                continue;
            }
        }
    }

    Ok(WordList { language, words })
}
