use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use console::Term;
use lingua::Language;
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
    let mut invalid_flag = false;
    let mut language = None;
    loop {
        let mut buf = String::new();
        if invalid_flag {
            print!("invalid language! input one among {LANGUAGES:?}: ");
        } else {
            print!("input to select a language: ");
        }
        stdout().flush()?;
        stdin().read_line(&mut buf)?;
        let tmp = buf.trim();
        if tmp == "" {
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

    let mut words = Vec::new();
    for i in 1.. {
        let mut buf = String::new();
        print!("input word {i}, or q to quit: ");
        stdout().flush()?;
        stdin().read_line(&mut buf)?;
        let tmp = buf.trim();
        if tmp == "q" || tmp == "" {
            break;
        }
        words.push(buf.trim().to_owned());
    }

    Ok(WordList { language, words })
}
