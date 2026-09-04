#[derive(Clone)]
pub struct Lexer {
    pub source: Vec<char>,
    start: usize,
    tokens: Vec<TokenTypes>,
    current_position: usize,
}

#[derive(Clone, Debug)]
pub enum TokenTypes {
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
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            start: 0,
            tokens: Vec::new(),
            current_position: 0,
        }
    }

    pub fn determine_token(&mut self) {
        while !self.is_at_end() {
            self.start = self.current_position;

            let character = self.source[self.current_position];

            match character {
                '{' => {
                    self.add_token(TokenTypes::LEFT_BRACE);
                    self.advance();
                }
                '}' => {
                    self.add_token(TokenTypes::RIGHT_BRACE);
                    self.advance();
                }
                '(' => {
                    self.add_token(TokenTypes::LEFT_PAREN);
                    self.advance();
                }
                ')' => {
                    self.add_token(TokenTypes::RIGHT_PAREN);
                    self.advance();
                }
                '+' => {
                    self.add_token(TokenTypes::PLUS);
                    self.advance();
                }
                '-' => {
                    self.add_token(TokenTypes::MINUS);
                    self.advance();
                }
                '*' => {
                    self.add_token(TokenTypes::STAR);
                    self.advance();
                }
                '.' => {
                    self.add_token(TokenTypes::DOT);
                    self.advance();
                }
                ';' => {
                    self.add_token(TokenTypes::SEMICOLON);
                    self.advance();
                }

                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::EQUAL_EQUAL);
                    } else {
                        self.add_token(TokenTypes::EQUAL);
                    }
                }

                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::LESS_EQUAL);
                    } else {
                        self.add_token(TokenTypes::LESS);
                    }
                }

                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::GREATER_EQUAL);
                    } else {
                        self.add_token(TokenTypes::GREATER);
                    }
                }

                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::BANG_EQUAL);
                    } else {
                        self.add_token(TokenTypes::BANG);
                    }
                }

                ' ' | '\r' | '\t' | '\n' => {
                    self.advance();
                }

                _ if self.is_number(character) => {
                    self.number();
                }

                _ if self.is_alpha(character) => {
                    self.identifier();
                }

                _ => {
                    panic!("Unexpected character: {}", character);
                }
            }
        }
    }

    pub fn add_token(&mut self, tokentype: TokenTypes) {
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

        self.add_token(TokenTypes::NUMBER);
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
    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current_position]
            .iter()
            .collect();

        let token = match text.as_str() {
            "and" => TokenTypes::AND,
            "or" => TokenTypes::OR,
            "not" => TokenTypes::NOT,
            "else" => TokenTypes::ELSE,
            "false" => TokenTypes::FALSE,
            "fn" => TokenTypes::FN,
            "for" => TokenTypes::FOR,
            "if" => TokenTypes::IF,
            "print" => TokenTypes::PRINT,
            "return" => TokenTypes::RETURN,
            "true" => TokenTypes::TRUE,
            "let" => TokenTypes::LET,
            "while" => TokenTypes::WHILE,
            _ => TokenTypes::IDENTIFIER,
        };

        self.add_token(token);
    }
    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }
    pub fn return_tokens(&self) -> Vec<TokenTypes> {
        return self.tokens.clone();
    }
}
