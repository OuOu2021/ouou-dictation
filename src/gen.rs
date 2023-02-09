use std::io::Write;

use anyhow::Result;
use language_tags::{LanguageTag, ParseError};
use lingua::{
    Language,
    Language::{Chinese, English, Japanese},
};
use tts::Tts;

pub const LANGUAGES: [Language; 3] = [English, Japanese, Chinese];

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Gender {
    Male,
    Female,
}

impl From<Gender> for tts::Gender {
    fn from(val: Gender) -> Self {
        match val {
            Gender::Male => tts::Gender::Male,
            Gender::Female => tts::Gender::Female,
        }
    }
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum Mode {
    Dictate,
    Read,
}

pub fn init_speaker(language: Language, gender: Gender, rate: f32) -> Result<tts::Tts> {
    let mut speaker = Tts::default()?;
    let voices = speaker.voices()?;

    if let Err(e) = speaker.set_voice(
        &voices
            .into_iter()
            .try_find(|x| {
                Ok::<bool, ParseError>(
                    x.gender().unwrap() == gender.into()
                        && LanguageTag::parse(&x.language())?.primary_language()
                            == LanguageTag::parse(&language.iso_code_639_1().to_string())?
                                .primary_language(),
                )
            })?
            .expect("No proper voice"),
    ) {
        panic!("Issue occurred when setting voice. {e:?}");
    }

    if let Err(e) = speaker.set_rate(rate) {
        panic!("Issue occurred when setting rate {e:?}");
    }

    Ok(speaker)
}

pub fn read(mut speaker: Tts, word_list: &[&str]) -> Result<()> {
    println!("Start Reading:");

    word_list.iter().enumerate().try_for_each(|(i, s)| {
        let s = s.trim();
        println!("{}. {s}", i + 1);
        speaker.speak(s, false)?;
        speaker.speak(s, false)?;
        while speaker.is_speaking()? {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok::<(), tts::Error>(())
    })?;

    println!("\n Read Over, about to quit");

    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

pub fn dictate(mut speaker: Tts, word_list: &Vec<&str>) -> Result<Vec<String>> {
    println!("Start Dictating:");
    let mut wrong_list = Vec::new();

    word_list.iter().enumerate().try_for_each(|(i, s)| {
        let s = s.trim();
        print!("{}. ", i + 1);
        std::io::stdout().flush().unwrap();

        speaker.speak(s, false)?;
        speaker.speak(s, false)?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input != s {
            println!("wrong!");
            wrong_list.push(format!("{input} -> {s}"));
        }
        speaker.stop()?;
        Ok::<(), tts::Error>(())
    })?;

    let cnt_words = word_list.len();
    let cnt_correct = cnt_words - wrong_list.len();

    println!("\nDictation Over.\n Accuracy:{cnt_correct}/{cnt_words}");

    Ok(wrong_list)
}

pub fn generate_wrong_list(wrong_list: Vec<String>, path: &str) -> anyhow::Result<()> {
    let mut output = Vec::new();

    wrong_list
        .into_iter()
        .try_for_each(|s| writeln!(output, "{s}"))?;
    std::fs::write(path, output)?;

    Ok(())
}
