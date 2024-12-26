use std::{env, path::PathBuf, process};

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

fn main() {
    let args = Arguments::parse();
    println!("{args:?}");
}
