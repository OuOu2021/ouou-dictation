use std::io::{stdout, Write};

use anyhow::{Context, Ok, Result};
use console::style;
use console::Term;
use indicatif::{ProgressBar, ProgressStyle};
use tts::Tts;

use crate::word_list::CorrectionList;

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
    /// Self-guided dictation
    Dictation,
    Speak,
    /// Build a word list
    BuildList,
}

pub fn read(speaker: &mut Tts, word_list: &[String]) -> Result<()> {
    println!("Start Reading:");

    word_list.iter().enumerate().try_for_each(|(i, s)| {
        let s = s.trim();
        println!("{}. {s}", i + 1);
        speaker.speak(s, false)?;
        speaker.speak(s, false)?;
        while speaker.is_speaking()? {
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok(())
    })?;

    println!("\n Read Over, about to quit");

    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}

pub fn dictate(term: &mut Term, speaker: &mut Tts, word_list: &[String]) -> Result<CorrectionList> {
    // Initialize Term & Progress Bar & Inputs
    let word_num = word_list.len();
    term.clear_screen()?;

    let pb = ProgressBar::new(word_num as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{bar:60}] {pos}/{len} {elapsed}")?
            .progress_chars("=> "),
    );
    let mut inputs = vec!["".to_string(); word_num];

    // Dictation
    word_list.iter().enumerate().try_for_each(|(i, s)| {
        term.move_cursor_to(0, 0)?;
        pb.set_position(i as u64);
        pb.set_message(format!("Dictating {}th word", i + 1));
        term.move_cursor_to(0, i + 1)?;
        let s = s.trim();
        print!("{}. ", i + 1);
        std::io::stdout().flush()?;

        speaker.speak(s, false)?;
        speaker.speak(s, false)?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        inputs[i] = input.to_owned();
        speaker.stop()?;
        Ok(())
    })?;
    term.move_cursor_to(0, 0)?;
    pb.finish();

    // Make changes
    let mut invalid_flag = false;
    loop {
        term.move_cursor_to(0, word_num + 3)?;
        term.clear_line()?;
        term.move_cursor_to(0, word_num + 2)?;
        term.clear_line()?;
        if !invalid_flag {
            print!("Any changes? input a number for the position to change, or q to quit: ");
        } else {
            print!("Invalid input, please enter a number, or q to quit: ");
        }
        stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.trim() == "" || input.trim() == "q" {
            break;
        }
        let number = input.trim().parse::<usize>();
        match number {
            Result::Ok(number) if (1..=word_num).contains(&number) => {
                let index = number - 1;
                print!("change {} to: ", inputs[index]);
                stdout().flush()?;
                let mut change = String::new();
                std::io::stdin().read_line(&mut change)?;
                let change = change.trim();
                term.move_cursor_to(0, number)?;
                term.clear_line()?;
                print!("{}. {}", number, style(change).yellow());
                stdout().flush()?;
                inputs[index] = change.to_owned();
            }
            _ => {
                invalid_flag = true;
                continue;
            }
        }
        invalid_flag = false;
    }

    term.move_cursor_to(0, word_num + 2)?;
    let mut cor_list = CorrectionList {
        words_and_correction: Vec::new(),
    };
    word_list.iter().enumerate().try_for_each(|(index, w)| {
        let number = index + 1;
        term.move_cursor_to(0, index + 1)?;
        term.clear_line()?;
        if inputs[index] != *w {
            // println!("wrong!");
            cor_list
                .words_and_correction
                .push((number, inputs[index].to_owned(), w.to_owned()));
            println!("{}. {} -> {}", number, style(&inputs[index]).red(), w);
        } else {
            println!("{}. {}", number, style(w).green());
        }
        Ok(())
    })?;

    // Result
    println!();
    let right = (word_list.len() - cor_list.words_and_correction.len()) as u64;
    let accuracy = right as f64 / word_num as f64 * 100.0;

    pb.set_length(100);
    pb.set_position(accuracy as u64);

    let style = if accuracy > 80.0 {
        ProgressStyle::default_bar().template("{msg} {bar:.green} {percent:.green}%")?
        // .progress_chars("=> ")
        // .tick_chars("██")
    } else {
        ProgressStyle::default_bar().template("{msg} {bar:.red} {percent:.red}%")?
        // .progress_chars("=> ")
        // .tick_chars("██")
    };
    pb.set_style(style);
    pb.abandon_with_message("Done. Accuracy: ");
    println!();
    Ok(cor_list)
}

pub fn generate_wrong_list(cor_list: CorrectionList, path: &str) -> Result<()> {
    std::fs::write(path, serde_json::to_string_pretty(&cor_list)?)
        .context(format!("fail to write correction list to {path}"))
}
