use std::collections::HashMap;

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let keywords = Scanner::initialize_keywords();

        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: keywords,
        }
    }
    fn initialize_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            return Err("There are some errors in the scanner".to_string());
        }
        Ok(self.tokens.clone())
    }

    fn is_at_end(self: &Self) -> bool {
        return self.current >= self.source.len();
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '!' => {
                let token = if self.char_match('=') {
                    return Ok(self.add_token(TokenType::BangEqual));
                } else {
                    return Ok(self.add_token(TokenType::Bang));
                };
            }
            '=' => {
                let token = if self.char_match('=') {
                    return Ok(self.add_token(TokenType::EqualEqual));
                } else {
                    return Ok(self.add_token(TokenType::Equal));
                };
            }
            '<' => {
                let token = if self.char_match('=') {
                    return Ok(self.add_token(TokenType::LessEqual));
                } else {
                    return Ok(self.add_token(TokenType::Less));
                };
            }
            '>' => {
                let token = if self.char_match('=') {
                    return Ok(self.add_token(TokenType::GreaterEqual));
                } else {
                    return Ok(self.add_token(TokenType::Greater));
                };
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break Ok(());
                        }
                        self.advance();
                    }
                } else {
                    Ok(self.add_token(TokenType::Slash))
                }
            }
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => Ok(self.line += 1),
            '"' => Ok(self.string()?),
            _ => {
                if c.is_digit(10) {
                    Ok(self.number()?)
                } else if is_alpha(c) {
                    Ok(self.identifier()?)
                } else {
                    return Err(format!("Unrecognized char: {}", c));
                }
            }
        }
    }
    fn identifier(self: &mut Self) -> Result<(), String> {
        while is_alpha(self.peek()) {
            self.advance();
        }

        let identifier: String = self
            .source
            .get(self.start..self.current)
            .map_or("", |s| s)
            .to_string();

        match self.keywords.get(&identifier) {
            Some(reserved) => {
                self.add_token(reserved.clone());
                Ok(())
            }
            None => {
                self.add_token(TokenType::Identifier);
                Ok(())
            }
        }
    }
    fn number(self: &mut Self) -> Result<(), String> {
        loop {
            if self.peek().is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            //consume the "."
            self.advance();

            loop {
                if self.peek().is_digit(10) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        let number_literal: String = self
            .source
            .get(self.start..self.current)
            .unwrap_or("")
            .to_string();

        let value: f64 = number_literal
            .parse()
            .unwrap_or_else(|e| panic!("Error converting to float: {}", e));

        self.add_token_with_literal(TokenType::Number, Some(LiteralValue::FloatValue(value)));
        Ok(())
    }
    fn peek_next(self: &Self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        };
        let c = self.source.as_bytes()[self.current + 1] as char;
        return c;
    }
    fn string(self: &mut Self) -> Result<(), String> {
        loop {
            if self.peek() != '"' && !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1
                };
                self.advance();
            } else {
                break;
            }
        }

        if self.is_at_end() {
            return Err("Undetermined string.".to_string());
        }

        self.advance();

        let string_literal: String = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap_or("")
            .to_string();

        self.add_token_with_literal(
            TokenType::String,
            Some(LiteralValue::StringValue(string_literal)),
        );

        Ok(())
    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        return self.source.as_bytes()[self.current] as char;
    }

    fn char_match(self: &mut Self, char_to_check: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        let c = self.source.as_bytes()[self.current] as char;
        if c != char_to_check {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }
    fn advance(self: &mut Self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }
    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(
        self: &mut Self,
        token_type: TokenType,
        literal: Option<LiteralValue>,
    ) {
        let text: String = self
            .source
            .get(self.start..self.current)
            .unwrap_or("")
            .to_string();

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        })
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
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
    Identifier,
    String,
    Number,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    False,
    FUN,
    FOR,
    IF,
    Nil,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    True,
    VAR,
    WHILE,

    EOF,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);

        assert_eq!(scanner.tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);

        assert_eq!(scanner.tokens[1].token_type, TokenType::BangEqual);
    }

    #[test]
    fn handle_string_literal() {
        let source = "\"ABC\" !";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens[0].token_type, TokenType::String);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Bang);
        assert_eq!(scanner.tokens[2].token_type, TokenType::EOF);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal value"),
        }
    }

    #[test]
    fn handle_string_literal_unterminated() {
        let source = r#""ABC"#;
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();
        match result {
            Err(_) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn handle_string_literal_multiline() {
        let source = "\"ABC\nabc\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();

        assert_eq!(scanner.tokens[0].token_type, TokenType::String);
        assert_eq!(scanner.tokens[1].token_type, TokenType::EOF);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val, "ABC\nabc"),
            _ => panic!("Incorrect literal value"),
        }
    }
    #[test]
    fn handle_number_literal_multiline() {
        let source = "12.3\n33.33\n1444.12";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].token_type, TokenType::Number);

        match scanner.tokens[0].literal {
            Some(LiteralValue::FloatValue(val)) => assert_eq!(val, 12.3),
            _ => panic!("Incorrect float value"),
        }
        match scanner.tokens[1].literal {
            Some(LiteralValue::FloatValue(val)) => assert_eq!(val, 33.33),
            _ => panic!("Incorrect float value"),
        }
        match scanner.tokens[2].literal {
            Some(LiteralValue::FloatValue(val)) => assert_eq!(val, 1444.12),
            _ => panic!("Incorrect float value"),
        }
    }
    #[test]
    fn handle_identifiers() {
        let source = "var verm_at = 23.3";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        println!("\n{:?}\n", scanner.tokens);
        assert_eq!(scanner.tokens[0].token_type, TokenType::VAR);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Identifier);
        assert_eq!(scanner.tokens[2].token_type, TokenType::Equal);
        assert_eq!(scanner.tokens[3].token_type, TokenType::Number);
    }
    #[test]
    fn handle_reserved_keywords() {
        let source = "and or";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens[0].token_type, TokenType::AND);
        assert_eq!(scanner.tokens[1].token_type, TokenType::OR);
    }
}
