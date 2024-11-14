mod config;
mod dictation;
mod word_list;
use config::*;
use dictation::*;

use anyhow::{Context, Result};
use clap::Parser;
use lingua::LanguageDetectorBuilder;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::process::exit;
use word_list::WordList;

use std::io;

fn main() -> Result<()> {
    let config = Config::parse();
    let input = std::fs::read_to_string(&config.path).context(format!(
        "failed to get input from {:?}",
        config.path.to_str().context("illegal unicode file name"),
    ))?;
    let mut input: WordList = serde_json::from_str(&input)?;

    let mut words = input.words;
    if input.language.is_none() {
        let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
        let detected = words.join(" ");
        input.language = detector.detect_language_of(detected);
    }

    if !config.dont_shuffle && config.mode == Mode::Dictate {
        let d = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .context("Duration since UNIX_EPOCH failed")?;
        let mut rng = rand::rngs::StdRng::seed_from_u64(d.as_secs());
        words.shuffle(&mut rng);
    }

    let speaker = init_speaker(input.language.unwrap(), config.gender, config.speed)?;

    match config.mode {
        Mode::Dictate => {
            let wrong_list = dictate(speaker, &words)?;
            let output = "./wrong_list.txt";

            if !wrong_list.is_empty() {
                generate_wrong_list(wrong_list, output).context("failed to generate wrong list")?;

                println!("Please check {output} for wrong words");
            }

            println!("About to quit.");

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Mode::Read => read(speaker, &words)?,
    };
    Ok(())
}
