mod lexer;
use crate::lexer::lexer::Lexer;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("main.fl")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut lexer = Lexer { line: line };
        let words = lexer.stripper();
        println!("{:?}", words);
    }
    Ok(())
}
