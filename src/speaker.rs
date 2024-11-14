use std::{
    io::{stdin, stdout, Write},
    num::ParseIntError,
};

use console::Term;
use lingua::Language;
use tts::{LanguageTag, Tts};

// use crate::Gender;
use anyhow::{Context, Result};

pub fn init_speaker(
    term: &mut Term,
    language: Language,
    // gender: Gender,
    rate: f32,
) -> Result<tts::Tts> {
    let mut speaker = Tts::default()?;
    let voices = speaker.voices()?;
    let mut proper_voices = Vec::new();
    for x in voices {
        if
        // x.gender().unwrap() == gender.into() &&
        LanguageTag::parse(x.language())
            .context("Parse Error")?
            .primary_language()
            == LanguageTag::parse(language.iso_code_639_1().to_string())
                .context("Parse Error")?
                .primary_language()
        {
            proper_voices.push(x);
        }
    }

    if proper_voices.is_empty() {
        return Err(anyhow::anyhow!("No proper voice"));
    } else {
        for (i, v) in proper_voices.iter().enumerate() {
            println!(
                "{}. name: {}, gender: {}, language: {}",
                i + 1,
                v.name(),
                match v.gender() {
                    Some(x) => format!("{x:?}"),
                    None => "None".to_string(),
                },
                v.language()
            );
        }
        let mut invalid_flag = false;
        loop {
            if invalid_flag {
                print!(
                    "invalid number, please enter a number from 1 to {}: ",
                    proper_voices.len()
                );
            } else {
                print!("input a number to select voice, or just enter to use the first one: ");
            }

            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let number: Result<usize, ParseIntError> = if input.trim() == "" {
                Result::Ok(1)
            } else {
                input.trim().parse::<usize>()
            };
            match number {
                Result::Ok(number) if (1..=proper_voices.len()).contains(&number) => {
                    speaker
                        .set_voice(&proper_voices[number - 1])
                        .context("fail to set voice")?;
                    break;
                }
                _ => {
                    term.move_cursor_up(1)?;
                    term.clear_line()?;
                    invalid_flag = true;
                    continue;
                }
            }
        }
    }
    speaker.set_rate(rate)?;
    Ok(speaker)
}
