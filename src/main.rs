use std::env;
use std::error::Error;
use std::fs;
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
    file_path: String,
    count_type: Option<CountType>,
}

impl Config {
    pub fn from_arguments(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // ignore the binary name

        let (count_type, file_path) = match (args.next(), args.next()) {
            (Some(flag), Some(path)) if flag == "-c" => (Some(CountType::Bytes), path),
            (Some(flag), Some(path)) if flag == "-l" => (Some(CountType::Lines), path),
            (Some(flag), Some(path)) if flag == "-w" => (Some(CountType::Words), path),
            (Some(flag), Some(path)) if flag == "-m" => (Some(CountType::Characters), path),
            (Some(_), Some(_)) => return Err("Invalid flag"),
            (Some(path), None) => (None, path),
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
    let filename = config.file_path.clone();
    let contents = fs::read_to_string(config.file_path)?;

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
