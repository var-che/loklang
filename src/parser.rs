use crate::expr::{Expr, Expr::*, LiteralValue};
use crate::scanner::{Token, TokenType, TokenType::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

macro_rules! match_tokens {
    ($parser:ident, $($token:ident),+) => {
        {
            let mut result = false;
            {
            $( result |= $parser.match_token($token); )*
            }

            result
        }

    };
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let rhs = self.comparison();
            let operator = self.previous();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator,
                right: Box::from(rhs),
            }
        }
        expr
    }
    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let rhs = self.term();

            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            }
        }
        expr
    }
    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[Minus, Plus]) {
            let op = self.previous();
            let rhs = self.factor();
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            };
        }
        expr
    }
    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(&[Slash, Star]) {
            let op = self.previous();
            let rhs = self.unary();
            expr = Binary {
                left: Box::from(expr),
                operator: op,
                right: Box::from(rhs),
            }
        }
        expr
    }
    fn unary(&mut self) -> Expr {
        let mut expr = self.unary();
        if self.match_tokens(&[Bang, Minus]) {
            let op = self.previous();
            let rhs = self.unary();
            Unary {
                operator: op,
                right: Box::from(rhs),
            }
        } else {
            self.primary()
        }
    }
    fn primary(&mut self) -> Expr {
        let token = self.peek();
        if self.match_token(&LeftParen) {
            let expr = self.expression();
            self.consume(RightParen, "Expected ')'");
            Grouping {
                expression: Box::from(expr),
            }
        } else {
            Literal {
                value: LiteralValue::from_token(token),
            }
        }
    }
    fn consume(&mut self, token_type: TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
        }
    }
    fn match_token(&self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == *typ {
                self.advance();
                true
            } else {
                false
            }
        }
    }
    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        todo!()
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn is_at_end(&self) -> bool {
        self.peek().token_type == EOF
    }
}
