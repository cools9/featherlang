mod lexer;

use crate::lexer::lexer::Lexer;

fn main() {
    let mut lexer = Lexer {
        line: String::from("let x = 42"),
    };

    let words = lexer.stripper();

    println!("{:?}", words);
}
