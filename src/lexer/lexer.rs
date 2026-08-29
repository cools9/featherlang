pub struct Lexer {
    pub line: String,
}

impl Lexer {
    pub fn stripper(&mut self) -> Vec<&str> {
        let words: Vec<&str> = self.line.split_whitespace().collect();
        words
    }
}
