#[derive(Clone)]
pub struct Lexer {
    pub source: Vec<char>,
    start: usize,
    tokens: Vec<TokenTypes>,
    current_position: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenTypes {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    Dot,
    Minus,
    Plus,
    Semicolon,
    SLASH,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

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
                    self.add_token(TokenTypes::LeftBrace);
                    self.advance();
                }
                '}' => {
                    self.add_token(TokenTypes::RightBrace);
                    self.advance();
                }
                '(' => {
                    self.add_token(TokenTypes::LeftParen);
                    self.advance();
                }
                ')' => {
                    self.add_token(TokenTypes::RightParen);
                    self.advance();
                }
                '+' => {
                    self.add_token(TokenTypes::Plus);
                    self.advance();
                }
                '-' => {
                    self.add_token(TokenTypes::Minus);
                    self.advance();
                }
                '*' => {
                    self.add_token(TokenTypes::Star);
                    self.advance();
                }
                '.' => {
                    self.add_token(TokenTypes::Dot);
                    self.advance();
                }
                ';' => {
                    self.add_token(TokenTypes::Semicolon);
                    self.advance();
                }

                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::EqualEqual);
                    } else {
                        self.add_token(TokenTypes::Equal);
                    }
                }

                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::LessEqual);
                    } else {
                        self.add_token(TokenTypes::Less);
                    }
                }

                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::GreaterEqual);
                    } else {
                        self.add_token(TokenTypes::Greater);
                    }
                }

                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        self.add_token(TokenTypes::BangEqual);
                    } else {
                        self.add_token(TokenTypes::Bang);
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
                '"' => {
                    self.string();
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
        let mut number = String::new();
        while self.is_number(self.peek()) {
            number.push(self.peek());
            self.advance();
        }

        if self.peek() == '.' && self.is_number(self.peek_next()) {
            self.advance();
            number.push('.');

            while self.is_number(self.peek()) {
                number.push(self.peek());
                self.advance();
            }
        }

        self.add_token(TokenTypes::NUMBER(number.parse().unwrap()));
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
            _ => TokenTypes::IDENTIFIER(text),
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

    fn string(&mut self) {
        let mut text = String::new();
        self.advance();
        while self.peek() != '"' && !self.is_at_end() {
            text.push(self.peek());
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string.");
        }

        self.advance();

        self.add_token(TokenTypes::STRING(text));
    }
}
