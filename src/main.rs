#![feature(try_find)]
mod cli;
mod gen;
use cli::*;
use gen::*;

use clap::Parser;
use lingua::LanguageDetectorBuilder;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use std::process::exit;

fn main() {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(&cli.path).unwrap_or_else(|e| {
        panic!(
            "failed to get input from {}: {e}",
            cli.path.to_str().unwrap_or_else(|| {
                eprintln!("illegal unicode file name");
                exit(1);
            })
        );
    });
    let detector = LanguageDetectorBuilder::from_languages(&LANGUAGES).build();
    let lang = detector.detect_language_of(&input).unwrap_or_else(|| {
        eprintln!("Can't determine the language of text");
        exit(1);
    });

    let mut input = input.lines().collect::<Vec<_>>();
    if !cli.dont_shuffle && cli.mode == Mode::Dictate {
        let d = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                eprintln!("Duration since UNIX_EPOCH failed: {e}");
                exit(1);
            });
        let mut rng = rand::rngs::StdRng::seed_from_u64(d.as_secs());
        input.shuffle(&mut rng);
    }

    let speaker = init_speaker(lang, cli.gender, cli.rate).unwrap_or_else(|e| {
        eprintln!("{e}");
        exit(1);
    });

    match cli.mode {
        Mode::Dictate => {
            let wrong_list = dictate(speaker, &input).unwrap_or_else(|e| panic!("{e}"));
            let output = "./wrong_list.txt";

            if !wrong_list.is_empty() {
                generate_wrong_list(wrong_list, output).unwrap_or_else(|e| {
                    eprintln!("failed to generate wrong list: {e}");
                    exit(1);
                });

                println!("Please check {output} for wrong words");
            }

            println!("About to quit.");

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Mode::Read => read(speaker, &input).unwrap_or_else(|e| {
            eprintln!("{e}");
            exit(1);
        }),
    };
}
