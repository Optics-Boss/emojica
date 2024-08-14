pub mod parser {
    use crate::{expr::expr::{Expr, LiteralValue}, parser_error, stmt::stmt::Stmt, token::token::{Token, TokenType}};

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
            let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string())?;

            let initializer = if matches!(self, TokenType::Equal) {
                Some(self.expression()?)
            } else {
                None
            };

            self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string())?;

            Ok(Stmt::Var { name, initializer })
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
                } else if let Expr::Get { object, name } = expr {
                    return Ok(Expr::Set { 
                        object,
                        name,
                        value 
                    })
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
                    right:  Box::new(expr)
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
                    expr = Expr::Get { object: Box::new(expr), name }
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
