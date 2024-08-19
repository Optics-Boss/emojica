pub mod stmt {
    use crate::{expr::expr::Expr, parser::parser::Error, token::token::Token};

    pub enum Stmt {
        Block { statements: Vec<Stmt> },
        Expression { expression: Expr },
        Function { 
            name: Token,
            params: Vec<Token>,
            body: Vec<Stmt>
        },
        If { 
            condition: Expr,
            else_branch: Box<Option<Stmt>>,
            then_branch: Box<Stmt>
        },
        Print { 
            expression: Expr,
        },
        Return {
            keyword: Token,
            value: Option<Expr>,
        },
        Var { 
            name: Token,
            initializer: Option<Expr>,
        },
        While { 
            condition: Expr,
            body: Box<Stmt>
        },
        Nil,
    }


    pub trait Visitor<R> {
        fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<R, Error>;
        fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_function_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<R, Error>;
        fn visit_if_stmt(&mut self, condition: &Expr, else_branch: &Option<Stmt>, then_branch: &Stmt) -> Result<R, Error>;
        fn visit_print_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Expr>) -> Result<R, Error>;
        fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<R, Error>;
        fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<R, Error>;
    }

    impl Stmt {
        pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
            match self {
                Stmt::Block { statements } => visitor.visit_block_stmt(statements),
                Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
                Stmt::Function { name, params, body } => { 
                    visitor.visit_function_stmt(name, params, body)
                },
                Stmt::If { condition, else_branch, then_branch } => {
                    visitor.visit_if_stmt(condition, else_branch, then_branch)
                },
                Stmt::Print { expression } => visitor.visit_print_stmt(expression),
                Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
                Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
                Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
                Stmt::Nil => unimplemented!(),
            }
        }
    }
}
