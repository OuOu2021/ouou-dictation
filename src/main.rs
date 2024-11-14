mod config;
mod dictation;
mod word_list;
use config::*;
use console::Term;
use dictation::*;

use anyhow::{Context, Result};
use clap::Parser;
use lingua::LanguageDetectorBuilder;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use word_list::WordList;

fn main() -> Result<()> {
    let mut term = Term::stdout();
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

    let mut speaker = init_speaker(input.language.unwrap(), config.gender, config.speed)?;

    match config.mode {
        Mode::Dictate => {
            let cor_list = dictate(&mut term, &mut speaker, &words)?;
            let output = "./wrong_list.txt";

            if !cor_list.is_empty() {
                term.clear_line()?;
                generate_wrong_list(cor_list, output).context("failed to generate wrong list")?;
                println!("Please check {output} for wrong words");
            }
            term.clear_line()?;
            println!("About to quit.");

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Mode::Read => read(&mut speaker, &words)?,
    };
    Ok(())
}
