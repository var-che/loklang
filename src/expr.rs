use crate::scanner::{Token, TokenType};

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }
}

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn to_string(&self) -> String {
        match self {
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string(); // TODO (*right) seems to recursively call
                                                      // the function, investigate that.
                format!("({} {})", operator_str, right_str)
            }
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let left_str = (*left).to_string();
                let right_str = (*right).to_string();

                format!("({} {} {})", operator_str, left_str, right_str)
            }
        }
    }
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token = Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 1,
        };

        let onetwothree = Expr::Literal {
            value: LiteralValue::Number(123.0),
        };

        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        };
        let multi = Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 1,
        };
        let ast = Expr::Binary {
            left: Box::from(Expr::Unary {
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi,
            right: Box::from(group),
        };
        let res = ast.to_string();
        assert_eq!(res, "(* (- 123) (group 45.67))");
    }
}
