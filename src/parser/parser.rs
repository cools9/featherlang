use crate::lexer::lexer::TokenTypes;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<TokenTypes>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenTypes>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
        }
    }
    fn peek(self) -> TokenTypes {
        <TokenTypes as Clone>::clone(&self.tokens[self.pos + 1])
    }

    fn consume(&mut self) -> TokenTypes {
        let current_token = self.clone().peek().clone();
        if current_token != TokenTypes::EOF {
            self.pos += 1;
        }
        current_token
    }
}
