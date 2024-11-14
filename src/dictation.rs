use std::io::{stdout, Write};

use anyhow::Result;
use console::style;
use console::Term;
use crossterm::{cursor, terminal, ExecutableCommand};
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
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
            .find(|x| {
                x.gender().unwrap() == gender.into()
                    && LanguageTag::parse(&x.language())
                        .expect("Parse Error")
                        .primary_language()
                        == LanguageTag::parse(&language.iso_code_639_1().to_string())
                            .expect("Parse Error")
                            .primary_language()
            })
            .expect("No proper voice"),
    ) {
        panic!("Issue occurred when setting voice. {e:?}");
    }

    if let Err(e) = speaker.set_rate(rate) {
        panic!("Issue occurred when setting rate {e:?}");
    }

    Ok(speaker)
}

pub fn read(mut speaker: Tts, word_list: &Vec<String>) -> Result<()> {
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

pub fn dictate(mut speaker: Tts, word_list: &Vec<String>) -> Result<Vec<String>> {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    let pb = ProgressBar::new(word_list.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{wide_bar}] {pos}/{len}")?
            .progress_chars("=> "),
    );
    let mut wrong_list = Vec::new();

    word_list.iter().enumerate().try_for_each(|(i, s)| {
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        pb.set_position(i as u64);
        pb.set_message(format!("Dictating {}th word", i + 1));
        stdout.execute(cursor::MoveTo(0, i as u16 + 1)).unwrap();
        let s = s.trim();
        print!("{}. ", i + 1);
        std::io::stdout().flush().unwrap();

        speaker.speak(s, false)?;
        speaker.speak(s, false)?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        // stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        // stdout
        //     .execute(terminal::Clear(terminal::ClearType::CurrentLine))
        //     .unwrap();
        let input = input.trim();
        if input != s {
            // println!("wrong!");
            wrong_list.push(format!("{input} -> {s}"));
        }
        speaker.stop()?;
        Ok::<(), tts::Error>(())
    })?;

    let len = word_list.len() as u64;
    let right = (word_list.len() - wrong_list.len()) as u64;
    let accuracy = right as f64 / len as f64 * 100.0;

    pb.set_length(100);
    pb.set_position(accuracy as u64);

    let style = if accuracy > 80.0 {
        ProgressStyle::default_bar().template("{msg} [{wide_bar:.green}] {pos}%")?
        // .progress_chars("=> ")
        // .tick_chars("██")
    } else {
        ProgressStyle::default_bar().template("{msg} [{wide_bar:.red}] {pos}%")?
        // .progress_chars("=> ")
        // .tick_chars("██")
    };
    pb.set_style(style);
    pb.abandon_with_message(format!("Done. Accuracy: {:.2}%", accuracy));

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
