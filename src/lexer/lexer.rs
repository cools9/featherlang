pub struct Lexer {
    pub source: Vec<char>,

    pub tokens: Vec<TokenTypes>,
    current_position: usize,
    maybe_position: usize,
}

enum TokenTypes {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl Lexer {
    pub fn DetermineToken(&mut self) {
        let character: char = self.source[self.current_position];
        match character {
            '{' => self.addToken(TokenTypes::LEFT_BRACE),
            '}' => self.addToken(TokenTypes::RIGHT_BRACE),
            '(' => self.addToken(TokenTypes::LEFT_PAREN),
            ')' => self.addToken(TokenTypes::RIGHT_PAREN),
            '+' => self.addToken(TokenTypes::PLUS),
            '-' => self.addToken(TokenTypes::MINUS),
            '*' => self.addToken(TokenTypes::STAR),
            '.' => self.addToken(TokenTypes::DOT),
            ';' => self.addToken(TokenTypes::SEMICOLON),
            '=' => {
                if (self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::EQUAL_EQUAL);
                } else if !(self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::EQUAL);
                } else {
                    todo!();
                }
            }
            '>' => {
                if (self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::LESS_EQUAL);
                } else if !(self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::LESS);
                } else {
                    todo!();
                }
            }
            '<' => {
                if (self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::GREATER_EQUAL);
                } else if !(self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::GREATER);
                } else {
                    todo!();
                }
            }
            '!' => {
                if (self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::BANG_EQUAL);
                } else if !(self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::BANG);
                } else {
                    todo!();
                }
            }

            _ => todo!(),
        }
    }
    pub fn addToken(&mut self, tokentype: TokenTypes) {
        self.tokens.push(tokentype);
    }
    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current_position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_position >= self.source.len()
    }
}
