mod config;
mod dictation;
mod speaker;
mod word_list;

use config::*;
use console::Term;
use dictation::*;

use anyhow::{Context, Result};
use clap::Parser;
use lingua::LanguageDetectorBuilder;
use lingua::{
    Language,
    Language::{Chinese, English, Japanese},
};
use rand::seq::SliceRandom;
use rand::{random, SeedableRng};
use speaker::init_speaker;
use tts::Tts;
use word_list::{build_word_list, WordList};
pub const LANGUAGES: [Language; 3] = [English, Japanese, Chinese];

fn main() -> Result<()> {
    let mut term = Term::stdout();
    let config = Config::parse();
    let mut init_word_list_and_speaker = || -> anyhow::Result<(Tts, Vec<String>)> {
        let word_list = std::fs::read_to_string(&config.path).context(format!(
            "failed to get word list from \"{}\"",
            config.path.to_str().context("illegal unicode file name")?,
        ))?;
        let mut input: WordList = serde_json::from_str(&word_list)?;

        let mut words = input.words;
        if input.language.is_none() {
            let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
            let detected = words.join(" ");
            input.language = detector.detect_language_of(detected);
        }

        if !config.dont_shuffle && config.mode == Mode::Dictation {
            let seed: u64 = random();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            words.shuffle(&mut rng);
        }
        let speaker = init_speaker(
            &mut term,
            input.language.unwrap(),
            // config.gender,
            config.speed,
        )?;
        Ok((speaker, words))
    };

    match config.mode {
        Mode::Dictation => {
            let (mut speaker, words) = init_word_list_and_speaker()?;

            let cor_list = dictate(&mut term, &mut speaker, &words)?;
            let output = "./wrong_list.txt";

            if !cor_list.is_empty() {
                term.clear_line()?;
                generate_wrong_list(cor_list, output).context("failed to generate wrong list")?;
                println!("Please check {output} for wrong words");
            }
            term.clear_line()?;
        }
        Mode::Speak => {
            let (mut speaker, words) = init_word_list_and_speaker()?;
            read(&mut speaker, &words)?;
        }
        Mode::BuildList => {
            let list = build_word_list(&mut term)?;
            std::fs::write(&config.path, serde_json::to_string_pretty(&list)?).context(format!(
                "fail to write correction list to {:?}",
                config.path
            ))?;
            println!("word list generated in {:?}", config.path);
        }
    }
    println!("About to quit.");
    std::thread::sleep(std::time::Duration::from_secs_f32(1.3));
    Ok(())
}
