mod lexer;
use crate::lexer::lexer::Lexer;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let args = Args::parse();
    //println!("Hello {}!", args.name);
    RunProgram(args.filename);
}

fn RunProgram(path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut lexer = Lexer { line: line };
        let words = lexer.stripper();
        println!("{:?}", words);
    }
    Ok(())
}
