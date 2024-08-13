pub mod scanner {
    use std::char;
    use std::collections::HashMap;

    use crate::{error, token::token::{Token, TokenType}};

    pub struct Scanner {
        source: String,
        tokens: Vec<Token>,
        start: usize,
        current: usize,
        line: i32,
    }

    impl Scanner {
        pub fn new(source: String) -> Self {
            Self {
                source: source.to_owned(),
                tokens: Vec::new(),
                start: 0,
                current: 0,
                line: 1,
            }
        }

        pub fn scan_tokens(&mut self) -> &Vec<Token> {
            while !self.is_at_end() {
                self.start = self.current;
                self.scan_token();
            }

            self.tokens.push(Token::new(TokenType::Eof, "".to_string(), self.line));
            &self.tokens
        }

        fn scan_token(&mut self) {
            let character : char = self.advance();

            match character {
                 '(' => self.add_token(TokenType::LeftParen),
                 ')' => self.add_token(TokenType::RightParen),
                 '{' => self.add_token(TokenType::LeftBrace),
                 '}' => self.add_token(TokenType::RightBrace),
                 ',' => self.add_token(TokenType::Comma),
                 '.' => self.add_token(TokenType::Dot),
                 '-' => self.add_token(TokenType::Minus),
                 '+' => self.add_token(TokenType::Plus),
                 ';' => self.add_token(TokenType::Semicolon),
                 '*' => self.add_token(TokenType::Star),
                 '!' => {
                     if self.match_character('=') {
                         self.add_token(TokenType::BangEqual)
                     } else {
                         self.add_token(TokenType::Bang)
                     }
                 }
                 '=' => {
                     if self.match_character('=') {
                         self.add_token(TokenType::EqualEqual)
                     } else {
                         self.add_token(TokenType::Equal)
                     }
                 }
                 '<' => {
                     if self.match_character('=') {
                         self.add_token(TokenType::LessEqual)
                     } else {
                         self.add_token(TokenType::Less)
                     }
                 }
                 '>' => {
                     if self.match_character('=') {
                         self.add_token(TokenType::GreaterEqual)
                     } else {
                         self.add_token(TokenType::Greater)
                     }
                 }
                 '/' => {
                     if self.match_character('/') {
                         while self.peek() != '\n' && !self.is_at_end() {
                             self.advance();
                         }
                     } else {
                         self.add_token(TokenType::Slash)
                     }
                 }
                 ' ' | '\r' | '\t' => (),
                 '\n' => self.line += 1,
                 '"' => self.string(),
                 character => {
                     if character.is_digit(10) {
                         self.number()
                     } else if character.is_alphabetic() || character == '_' {
                         self.identifier()
                     } else {
                         error(self.line, "Unexpected character.".to_string())
                     }
                 }

            }
        }

        fn match_character(&mut self, expected: char) -> bool {
            if self.is_at_end() {
                return false;
            }

            if self.source
                    .chars()
                    .nth(self.current)
                    .expect("Unexpected end of source.") != expected {
                return false;
            }

            self.current += 1;
            true
        }

        fn identifier(&mut self) {
            let keywords : HashMap<&str, TokenType> = HashMap::from([
                ("and", TokenType::And),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]);

            while self.peek().is_alphanumeric() || self.peek() == '_' {
                self.advance();
            }

            let text = self
                .source
                .get(self.start..self.current)
                .expect("Unexpected end.");

            let token_type: TokenType = keywords
                .get(text)
                .cloned()
                .unwrap_or(TokenType::Identifier);

            self.add_token(token_type)
        }

        fn string(&mut self) {
            while self.peek() != '"' && !self.is_at_end() {

                if self.peek() == '\n' {
                    self.line += 1;
                }

                self.advance();
            }

            if self.is_at_end() {
                error(self.line, "Unterminated string.".to_string())
            }

            self.advance();

            let literal = self
                .source
                .get((self.start + 1)..(self.current - 1))
                .expect("Unexpected end.")
                .to_string();

            self.add_token(TokenType::String { literal })
        }

        fn number(&mut self) {
            while self.peek().is_digit(10) {
                self.advance();
            }

            if self.peek() == '.' && self.peek_next().is_digit(10) {
                self.advance();

                while self.peek().is_digit(10) {
                    self.advance();
                }
            }

            let number : f64 = self
                .source
                .get(self.start..self.current)
                .expect("Unexpected end.")
                .parse()
                .expect("Scanned number could not be parsed.");

            self.add_token(TokenType::Number { literal: number })
        }


        fn advance(&mut self) -> char {
            self.current += 1;
            let character_vector : Vec<char> = self.source.chars().collect();
            character_vector[self.current - 1]
        }

        fn peek(&self) -> char {
            self.source.chars().nth(self.current).unwrap_or('\0')
        }

        fn peek_next(&self) -> char {
            self.source.chars().nth(self.current + 1).unwrap_or('\0')
        }

        fn is_at_end(&self) -> bool {
            self.current >= self.source.len()
        }

        fn add_token(&mut self, token_type: TokenType) {
            let text = self.source.get(self.start..self.current).expect("No token");
            self.tokens.push(Token::new(token_type, text.to_string(), self.line))

        }

    }

}
