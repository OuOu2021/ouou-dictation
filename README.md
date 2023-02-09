# ouou-dictation
A command-line program for **self-help dictation** supporting Chinese, Japanese and English.

It's the first product of my Rust learning. Mostly for my own use, but if you find any issues or have good ideas, just let me know.

## Features
* Use [clap](https://crates.io/crates/clap) to parse arguments

* Use [Lingua](https://crates.io/crates/lingua) to detect languages automatically.

* Use [TTS-RS](https://crates.io/crates/tts) to speak out the words in various backends.

* Two Modes:
  * Dictate Mode for Self-dictation. Check the answer in real time
  * Read Mode just read over the word list

* Switch Male/Female

* Switch whether to shuffle the word list

* Set speaking rate

## Usage
Build it or download the release.

Run it in the console. take `--help` as argument for help, like:
```
PS C:\Users\OuOu\Desktop> .\ouou_dictation.exe --help
A command-line program for multi-language self-help dictation.

Usage: ouou_dictation.exe [OPTIONS] --path <PATH> [MODE] [GENDER]

Arguments:
  [MODE]    [default: dictate] [possible values: dictate, read]
  [GENDER]  [default: female] [possible values: male, female]

Options:
  -p, --path <PATH>
  -r, --rate <RATE>   Use like 0.5/2.0 [default: 0.9]
  -d, --dont-shuffle  Do not shuffle the word list
  -h, --help          Print help
  -V, --version       Print version
```

If you want to use it for a long time, it is more convenient to add the exe to the environment path. 

It don't have a shorter name like rg(ripgrep) currently.
