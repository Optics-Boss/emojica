pub mod parser {
    use std::{convert, fmt, io};

    use crate::{expr::expr::{Expr, LiteralValue}, object::object::Object, parser_error, stmt::stmt::Stmt, token::token::{Token, TokenType}};

    #[derive(Debug)]
    pub enum Error {
        Io(io::Error),
        Parse,
        Runtime { token: Token, message: String },
        Return { value: Object }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Io(underlying) => write!(f, "IoError {}", underlying),
                Error::Parse => write!(f, "ParseError"),
                Error::Return { value } => write!(f, "Return {:?}", value),
                Error::Runtime { message, .. } => write!(f, "RuntimeError {}", message),
            }
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            "Lox Error"
        }
    }

    impl convert::From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Error::Io(e)
        }
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
                self.function("function".to_string())
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
            let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

            let initializer = if matches!(self, TokenType::Equal) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;

            Ok(Stmt::Var { name, initializer })
        }

        fn function(&mut self, kind: String) -> Result<Stmt, Error> {
            let name = self.consume(
                TokenType::Identifier,
                format!("Expect {} name.", kind)
            )?;

            self.consume(TokenType::LeftParen, format!("Expect '(' {} name.", kind))?;
            let mut params: Vec<Token> = Vec::new();

            if !self.check(TokenType::RightParen) {
                loop {
                    if params.len() >= 255 {
                        self.error(self.peek(), "Cannot have more than 255 parameters.".to_string());
                    }

                    params.push(self.consume(TokenType::Identifier, "Expect parameter name.".to_string())?);

                    if !matches!(self, TokenType::Comma) {
                        break;
                    }

                }
            }

            self.consume(TokenType::RightParen, "Expect ')'  name.".to_string())?;

            self.consume(
                TokenType::LeftBrace,
                format!("Expect '{{' before {} body.", kind)
            )?;

            let body = self.block()?;
            Ok(Stmt::Function { name, params, body })
        }

        fn block(&mut self) -> Result<Vec<Stmt>, Error> {
            let mut statements: Vec<Stmt> = Vec::new();

            while !self.check(TokenType::RightBrace) && !self.is_at_end() {
               statements.push(self.declaration()?);
            }

            self.consume(TokenType::RightBrace, "Expect '}' after block.".to_string())?;
            Ok(statements)
        }

        fn statement(&mut self) -> Result<Stmt, Error> {
            if matches!(self, TokenType::For) {
                self.for_statement()
            } else if matches!(self, TokenType::If) {
                self.if_statement()
            } else if matches!(self, TokenType::Print) {
                self.print_statement()
            } else if matches!(self, TokenType::Return) {
                self.return_statement()
            } else if matches!(self, TokenType::While) {
                self.while_statement()
            } else if matches!(self, TokenType::LeftBrace) {
                Ok(Stmt::Block { statements: self.block()? })
            } else {
                self.expression_statement()
            }
        }

        fn for_statement(&mut self) -> Result<Stmt, Error> {
            self.consume(TokenType::LeftParen, "Expect '(' after 'for'.".to_string())?;

            let initializer = if matches!(self, TokenType::Semicolon) {
                None
            } else if matches!(self, TokenType::Var) {
                Some(self.var_declaration()?)
            } else {
                Some(self.expression_statement()?)
            };

            let condition = if !self.check(TokenType::Semicolon) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::Semicolon, "Expect ';' after loop condition".to_string())?;

            let increment = if !self.check(TokenType::RightParen) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::RightParen, "Expect ')' after for clauses.".to_string())?;

            let mut body = self.statement()?;

            if let Some(inc) = increment {
                let inc_stmt = Stmt::Expression { expression: inc };
                body = Stmt::Block { statements: vec![body, inc_stmt] }
            }

            body = Stmt::While {
                condition: condition.unwrap_or(Expr::Literal {
                    value: LiteralValue::Boolean(true),
                }),
                body: Box::new(body),
            };

            if let Some(init_stmt) = initializer {
                body = Stmt::Block { statements: vec![init_stmt, body] }
            }

            Ok(body)
        }

        fn if_statement(&mut self) -> Result<Stmt, Error> {
            self.consume(TokenType::LeftParen, "Expect '(' after 'if'. ".to_string())?;
            let condition = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after if condition".to_string())?;

            let then_branch = Box::new(self.statement()?);
            let else_branch = if matches!(self, TokenType::Else) {
                Box::new(Some(self.statement()?))
            } else {
                Box::new(None)
            };

            Ok(Stmt::If {
                condition,
                else_branch,
                then_branch,
            })
        }

        fn print_statement(&mut self) -> Result<Stmt, Error> {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;
            Ok(Stmt::Print { expression: value })
        }

        fn return_statement(&mut self) -> Result<Stmt, Error> {
            let keyword: Token = self.previous().clone();
            let value = if !self.check(TokenType::Semicolon) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::Semicolon, "Expect ';' after return values".to_string())?;
            Ok(Stmt::Return { keyword, value })
        }

        fn while_statement(&mut self) -> Result<Stmt, Error> {
            self.consume(TokenType::LeftParen, "Expect '(' after 'while'.".to_string())?;
            let condition = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after condition.".to_string())?;
            let body = Box::new(self.statement()?);

            Ok(Stmt::While { condition, body })
        }

        fn expression_statement(&mut self) -> Result<Stmt, Error> {
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after expression.".to_string())?;
            Ok(Stmt::Expression { expression: expr })
        }

        fn synchronize(&mut self) {
            self.advance();

            while !self.is_at_end() {
                if self.previous().token_type == TokenType::Semicolon {
                    return;
                }

                match self.peek().token_type {
                    TokenType::Fun |
                    TokenType::Var |
                    TokenType::For |
                    TokenType::If |
                    TokenType::While |
                    TokenType::Print |
                    TokenType:: Return => return,
                    _ => self.advance(),
                };
            }
        }


        fn expression(&mut self) -> Result<Expr, Error> {
            self.assignment()
        }

        fn assignment(&mut self) -> Result<Expr, Error> {
            let expr = self.or_expr()?;

            if matches!(self, TokenType::Equal) {
                let value = Box::new(self.assignment()?);

                if let Expr::Variable {name} = expr {
                    return Ok(Expr::Assign { name, value })
                } 

                let equals = self.previous();
                self.error(equals, "Invalid assignment target.".to_string());
            }



            Ok(expr)
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
                    right:  Box::new(right)
                }
            }

            Ok(expr)
        }

        fn comparison(&mut self) -> Result<Expr, Error> {
            let mut expr = self.addition()?;

            while matches!(
                self,
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual
            ) {
                let operator: Token = self.previous().clone();
                let right = self.addition()?;
                expr = Expr::Binary { 
                    left: Box::new(expr), 
                    operator, 
                    right: Box::new(right)
                }
            }

            Ok(expr)
        }

        fn addition(&mut self) -> Result<Expr, Error> {
            let mut expr = self.multiplication()?;

            while matches!(self, TokenType::Minus, TokenType::Plus) {
                let operator : Token = self.previous().clone();
                let right = self.multiplication()?;
                expr = Expr::Binary { 
                    left: Box::new(expr), 
                    operator,
                    right: Box::new(right),
                }
            }

            Ok(expr)
        }

        fn multiplication(&mut self) -> Result<Expr, Error> {
            let mut expr = self.unary()?;

            while matches!(self, TokenType::Slash, TokenType::Star) {
                let operator: Token = self.previous().clone();
                let right = self.unary()?;
                expr = Expr::Binary { 
                    left: Box::new(expr),
                    operator, 
                    right: Box::new(right),
                }
            }


            Ok(expr)
        }

        fn unary(&mut self) -> Result<Expr, Error> {
            if matches!(self, TokenType::Bang, TokenType::Minus) {
                let operator: Token = self.previous().clone();
                let right = self.unary()?;
                Ok(Expr::Unary {
                    operator,
                    right: Box::new(right)
                })
            } else {
                self.call()
            }
        }

        fn call(&mut self) -> Result<Expr, Error> {
            let mut expr = self.primary()?;
            
            loop {
                if matches!(self, TokenType::LeftParen) {
                    expr = self.finish_call(expr)?;
                } else if matches!(self, TokenType::Dot) {
                    let name = self.consume(TokenType::Identifier, "Expect property after '.'.".to_string())?;
                } else {
                    break;
                }
            }


            Ok(expr)
        }

        fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
            let mut arguments: Vec<Expr> = Vec::new();

            if !self.check(TokenType::RightParen) {
                loop {
                    if arguments.len() >= 255 {
                        self.error(self.peek(), "Cannot have more than 255 arguments".to_string());
                    }

                    arguments.push(self.expression()?);
                    if matches!(self, TokenType::Comma) {
                        break;
                    }
                }
            }

            let parent = self.consume(TokenType::RightParen, "Expect ')' after arguments.".to_string())?;
            Ok(Expr::Call {
                callee: Box::new(callee),
                paren: parent,
                arguments
            })
        }

        fn primary(&mut self) -> Result<Expr, Error> {
            let expr = match &self.peek().token_type {
                TokenType::False => Expr::Literal {
                    value: LiteralValue::Boolean(false),
                },
                TokenType::True => Expr::Literal {
                    value: LiteralValue::Boolean(true), 
                },
                TokenType::Nil => Expr::Literal { 
                    value:  LiteralValue::Null,
                },
                TokenType::String { literal } => Expr::Literal { 
                    value: LiteralValue::String(literal.clone())
                },
                TokenType::Number { literal } => Expr::Literal { 
                    value: LiteralValue::Number(literal.clone())
                },
                TokenType::Identifier => Expr::Variable { 
                    name: self.peek().clone()
                },
                TokenType::LeftParen => {
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expected ')' after expression.".to_string())?;
                    Expr::Grouping { 
                        expression: Box::new(expr)
                    }
                },
                _ => return Err(self.error(self.peek(), "Expect expression.".to_string()))
            };

            self.advance();
            Ok(expr)
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
