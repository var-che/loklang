use crate::expr::*;
use crate::scanner::{Token, TokenType::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality();
    }

    fn equality(&mut self) -> Expr {
        let lhs = self.comparison();

        while self.match(BangEqual, EqualEqual) {
            let operator: Token = self.previous();
            let rhs: Expr = self.comparison();

        }

        todo!()
    }
}
