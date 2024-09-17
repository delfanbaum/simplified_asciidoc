use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser as ClapParser;
use simplified_asciidoc::parse::Parser;

#[derive(ClapParser, Debug)]
#[command(
    name = "sdoc",
    version,
    about,
    long_about = "A rust coverter from simplified asciidoc to html."
)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    match open(&args.file) {
        Err(e) => eprintln!("Failed to open {}: {}", args.file, e),
        Ok(file) => {
            let mut parser = Parser::new();
            for line in file.lines() {
                // note, we're ignoring a bunch of errors here
                parser.parse_line(&line.unwrap());
            }
        }
    }

    // Read the file line by line
    // Use each line to determine the block kind
    // Once you have the block structure, go tag-by-tag and do inlines?
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
