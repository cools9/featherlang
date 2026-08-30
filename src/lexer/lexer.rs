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
            _ => todo!(),
        }
        self.advance();
    }
    pub fn addToken(&mut self, tokentype: TokenTypes) {
        self.tokens.push(tokentype);
    }
    pub fn advance(&mut self) {
        self.current_position += 1;
    }
}
