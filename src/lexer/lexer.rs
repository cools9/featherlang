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
    OR,
    NOT,
    ELSE,
    FALSE,
    FN,
    FOR,
    IF,
    PRINT,
    RETURN,
    TRUE,
    LET,
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
            '<' => {
                if (self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::LESS_EQUAL);
                } else if !(self.source[self.current_position + 1] == '=') {
                    self.addToken(TokenTypes::LESS);
                } else {
                    todo!();
                }
            }
            '>' => {
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
            'a' => {
                if (self.peek_next() == 'n' && self.source[self.current_position + 2] == 'd') {
                    self.addToken(TokenTypes::AND);
                } else {
                    todo!();
                }
            }

            'o' => {
                if (self.peek_next() == 'r' && self.source[self.current_position + 2] == 'd') {
                    self.addToken(TokenTypes::OR);
                } else {
                    todo!();
                }
            }

            'l' => {
                if (self.peek_next() == 'e' && self.source[self.current_position + 2] == 't') {
                    self.addToken(TokenTypes::LET);
                } else {
                    todo!();
                }
            }
            'i' => {
                if (self.peek_next() == 'f') {
                    self.addToken(TokenTypes::IF);
                } else {
                    todo!();
                }
            }

            'e' => {
                if (self.peek_next() == 'l'
                    && self.source[self.current_position + 2] == 's'
                    && self.source[self.current_position + 3] == 'e')
                {
                    self.addToken(TokenTypes::ELSE);
                } else {
                    todo!();
                }
            }

            't' => {
                if (self.peek_next() == 'r'
                    && self.source[self.current_position + 2] == 'u'
                    && self.source[self.current_position + 3] == 'e')
                {
                    self.addToken(TokenTypes::TRUE);
                } else {
                    todo!();
                }
            }

            _ if self.is_number(character) => self.number(),
            ' ' | '\r' | '\t' | '\n' => {}

            _ => todo!("tf am i doing vro"),
        }
    }
    pub fn addToken(&mut self, tokentype: TokenTypes) {
        self.tokens.push(tokentype);
    }
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current_position += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current_position >= self.source.len()
    }

    fn number(&mut self) {
        while self.is_number(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_number(self.peek_next()) {
            self.advance();

            while self.is_number(self.peek()) {
                self.advance();
            }
        }

        self.addToken(TokenTypes::NUMBER);
    }

    fn is_number(&self, character: char) -> bool {
        let character_position: u8 = character as u8;
        if character_position >= 48 && character_position <= 57 {
            return true;
        } else {
            return false;
        }
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current_position]
        }
    }

    fn peek_next(&self) -> char {
        if self.current_position + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current_position + 1]
        }
    }
}
