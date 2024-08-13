pub mod parser {
    use crate::{expr::expr::Expr, parser_error, stmt::stmt::Stmt, token::token::{Token, TokenType}};

    #[derive(Debug)]
    pub enum Error {
        Parse,
    }

    pub struct Parser {
        tokens: Vec<Token>,
        current: usize,
    }

    macro_rules! matches {
        ( $sel:ident, $( $x:expr ),* ) => {
            {
                if $( $sel.check($x) )||* {
                    $sel.advance();
                    true
                } else {
                    false
                }
            }
        };
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Self {
            Parser {tokens, current: 0}
        }

        pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
            let mut statements: Vec<Stmt> = Vec::new();
            while !self.is_at_end() {
                statements.push(self.declaration()?);
            }

            Ok(statements)
        }

        fn declaration(&mut self) -> Result<Stmt, Error> {
            let statement = if matches!(self, TokenType::Var) {
                self.var_declaration()
            } else if matches!(self, TokenType::Fun) {
                self.function("function")
            } else {
                self.statement()
            };

            match statement {
                Err(Error::Parse) => {
                    self.synchronize();
                    Ok(Stmt::Nil)
                }
                other => other,
            }
        }

        fn var_declaration(&mut self) -> Result<Stmt, Error> {
            let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string());

            let initializer = if matches!(self, TokenType::Equal) {
                Some(self.expression())
            } else {
                None
            };
        }

        fn expression(&mut self) -> Result<Expr, Error> {
            self.assignment()
        }

        fn assignment(&mut self) -> Result<Expr, Error> {
            let expr = self.or_expr()
        }

        fn or_expr(&mut self) -> Result<Expr, Error> {
            let mut expr = self.and_expr()?;

            while matches!(self, TokenType::Or) {
                let operator: Token = (*self.previous()).clone();
                let right: Expr = self.and_expr()?;
                expr = Expr::Logical { 
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right) 
                }
            }

            Ok(expr)
        }

        fn and_expr(&mut self) -> Result<Expr, Error> {
            let mut expr = self.equality()?;

            while matches!(self, TokenType::And) {
                let operator: Token = (*self.previous()).clone();
                let right: Expr = self.equality()?;
                expr = Expr::Logical {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };
            }

            Ok(expr)        
        }

        fn equality(&mut self) -> Result<Expr, Error> {
            let mut expr = self.comparison()?;

            while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
                let operator: Token = (*self.previous()).clone();
                let right: Expr = self.comparison()?;
                expr = Expr::Binary { 
                    left: Box::new(expr), 
                    operator,
                    right:  Box::new(expr)
                }
            }

            Ok(expr)
        }

        fn comparison(&mut self) -> Result<Expr, Error> {

        }

        fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, Error> {
            if self.check(token_type) {
                Ok(self.advance().clone())
            } else {
                Err(self.error(self.peek(), message))
            }
        }

        fn check(&self, token_type: TokenType) -> bool {
            if self.is_at_end() {
                return false;
            }

            token_type == self.peek().token_type

        }

        fn error(&self, token: &Token, message: String) -> Error {
            parser_error(token, message);
            Error::Parse
        }

        fn advance(&mut self) -> &Token {
            if !self.is_at_end() {
                self.current += 1;
            }

            self.previous()
        }

        fn previous(&self) -> &Token {
            self.tokens
                .get(self.current - 1)
                .expect("Previous was empty")
        }

        fn peek(&self) -> &Token {
            self.tokens
                .get(self.current)
                .expect("Peek into end of token stream.")
        }

        fn is_at_end(&self) -> bool {
            self.peek().token_type == TokenType::Eof
        }
    }

}
