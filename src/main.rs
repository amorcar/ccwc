use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let config = Config::from_arguments(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub enum CountType {
    Bytes,
    Words,
    Lines,
    Characters,
}

pub struct Config {
    file_path: Option<String>,
    count_type: Option<CountType>,
}

impl Config {
    pub fn from_arguments(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore the binary name

        let (count_type, file_path) = match (args.next(), args.next()) {
            (Some(flag), Some(path)) if flag == "-c" => (Some(CountType::Bytes), Some(path)),
            (Some(flag), Some(path)) if flag == "-l" => (Some(CountType::Lines), Some(path)),
            (Some(flag), Some(path)) if flag == "-w" => (Some(CountType::Words), Some(path)),
            (Some(flag), Some(path)) if flag == "-m" => (Some(CountType::Characters), Some(path)),
            (Some(_), Some(_)) => return Err("Invalid flag"),
            (Some(arg), None) => {
                if arg == "-c" {
                    (Some(CountType::Bytes), None)
                } else if arg == "-l" {
                    (Some(CountType::Lines), None)
                } else if arg == "-w" {
                    (Some(CountType::Words), None)
                } else if arg == "-c" {
                    (Some(CountType::Characters), None)
                } else {
                    (None, Some(arg))
                }
            }
            _ => return Err("Invalid arguments"),
        };

        Ok(Config {
            file_path,
            count_type,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read config.filepath
    let (contents, filename) = match config.file_path {
        Some(fp) => (fs::read_to_string(&fp)?, fp.clone()),
        None => {
            let mut buffer = String::new();
            let _ = io::stdin().read_to_string(&mut buffer);
            (buffer, "".to_string())
        }
    };

    // depending on config.count_type do one count or another
    let result = match config.count_type {
        Some(CountType::Bytes) => format_result(count_bytes(&contents)),
        Some(CountType::Lines) => format_result(count_lines(&contents)),
        Some(CountType::Words) => format_result(count_words(&contents)),
        Some(CountType::Characters) => format_result(count_characters(&contents)),
        None => format!(
            "{} {} {} {}",
            count_lines(&contents),
            count_words(&contents),
            count_bytes(&contents),
            filename,
        ),
    };

    // print the result
    println!("{result}");

    Ok(())
}

pub fn count_bytes(contents: &str) -> usize {
    contents.as_bytes().len()
}

pub fn count_lines(contents: &str) -> usize {
    contents.lines().count()
}

pub fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}

pub fn count_characters(contents: &str) -> usize {
    contents.chars().count()
}

pub fn format_result(result: usize) -> String {
    format!("{result}")
}
