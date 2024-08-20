pub mod token {
    use std::hash::{Hash, Hasher};


    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenType {
        LeftParen, RightParen, LeftBrace, RightBrace,
        Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
        Bang, BangEqual, Equal, EqualEqual,Greater, GreaterEqual, Less, LessEqual,
        Identifier, String {literal: String}, Number{literal: f64},
        And, Else, False, True, Fun, For, If, Nil, Or, Print, Return, Var, While, Eof
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
        pub line: i32,
    }

    impl Token {
        pub fn new(token_type: TokenType, lexeme: String, line: i32) -> Self {
            Self {
                token_type,
                lexeme,
                line,
            }
        }
    }

    impl Hash for Token {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.lexeme.hash(state);
            self.line.hash(state);
        }
    }

    impl Eq for Token {}
}
