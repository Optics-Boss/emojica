pub mod token {

    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenType {
        LeftParen, RightParen, LeftBrace, RightBrace,
        Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
        Bang, BangEqual, Equal, EqualEqual,Greater, GreaterEqual, Less, LessEqual,
        Identifier, String {literal: String}, Number{literal: f64},
        And, Else, False, True, Fun, For, If, Nil, Or, Print, Return, Var, While, Eof
    }

    pub struct Token {
        token_type: TokenType,
        lexeme: String,
        line: i32,
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

}
