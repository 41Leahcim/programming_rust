use std::{
    borrow::Cow,
    env,
    fs::{self},
    path::PathBuf,
    process,
};

use regex::Regex;
use text_colorizer::Colorize;

fn print_usage() {
    eprintln!(
        "{} - change occurences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT_FILE> <OUTPUT_FILE>");
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: PathBuf,
    output: PathBuf,
}

impl Arguments {
    fn too_few_arguments() -> ! {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4.",
            "Error".red().bold()
        );
        process::exit(1);
    }

    pub fn parse() -> Self {
        let mut args = env::args().skip(1);
        Self {
            target: args.next().unwrap_or_else(|| Self::too_few_arguments()),
            replacement: args.next().unwrap_or_else(|| Self::too_few_arguments()),
            filename: PathBuf::from(args.next().unwrap_or_else(|| Self::too_few_arguments())),
            output: PathBuf::from(args.next().unwrap_or_else(|| Self::too_few_arguments())),
        }
    }
}

fn replace<'text>(
    target: &str,
    replacement: &str,
    text: &'text str,
) -> Result<Cow<'text, str>, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement))
}

fn main() {
    let args = Arguments::parse();
    println!("{args:?}");
    let text = fs::read_to_string(&args.filename).unwrap_or_else(|error| {
        eprintln!(
            "{} failed to open file for reading '{}': {error:?}",
            args.filename.to_string_lossy(),
            "Error:".red().bold()
        );
        process::exit(1);
    });
    let text = replace(&args.target, &args.replacement, &text).unwrap_or_else(|error| {
        eprintln!(
            "{} failed to replace text: {error:?}",
            "Error:".red().bold()
        );
        process::exit(1);
    });
    fs::write(&args.output, text.as_bytes()).unwrap_or_else(|error| {
        eprintln!(
            "{} failed to write to file '{}': {error:?}",
            "Error:".red().bold(),
            args.output.to_string_lossy()
        );
        process::exit(1);
    });
}
