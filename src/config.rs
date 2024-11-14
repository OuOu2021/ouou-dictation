use crate::dictation::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    #[arg(long, short)]
    pub path: std::path::PathBuf,

    #[arg(value_enum, default_value_t = Mode::Dictate)]
    pub mode: Mode,

    #[arg(value_enum, default_value_t = Gender::Female)]
    pub gender: Gender,

    #[arg(long, short, default_value_t = 0.9, help = "Use like 0.5 or 1.2")]
    pub speed: f32,

    #[arg(long, short, action = clap::ArgAction::SetTrue, help = "Do not shuffle the word list")]
    pub dont_shuffle: bool,
}
