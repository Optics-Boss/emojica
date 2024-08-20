pub mod expr {
    use crate::{parser::parser::Error, token::token::Token};
    
    #[derive(Clone, Debug)]
    pub enum Expr {
        Assign {
            name: Token,
            value: Box<Expr>
        },
        Binary {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>,
        },
        Call {
            callee: Box<Expr>,
            paren: Token,
            arguments: Vec<Expr>
        },
        Grouping {
            expression: Box<Expr>,
        },
        Literal {
            value: LiteralValue,
        },
        Logical {
            left: Box<Expr>,
            operator: Token,
            right: Box<Expr>
        },
        Unary {
            operator: Token,
            right: Box<Expr>
        },
        Variable {
            name: Token,
        }
    }

    #[derive(Clone, Debug)]
    pub enum LiteralValue {
        Boolean(bool),
        Null,
        Number(f64),
        String(String),
    }

    pub trait Visitor<R> {
        fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<R, Error>;
        fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
        fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> Result<R, Error>;
        fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_literal_expr(&mut self, value: &LiteralValue) -> Result<R, Error>;
        fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<R, Error>;
        fn visit_variable_expr(&mut self, name: &Token) -> Result<R, Error>;
    }

    impl Expr {
        pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
            match self {
                Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
                Expr::Binary { left, operator, right } => { 
                    visitor.visit_binary_expr(left, operator, right)
                },
                Expr::Call { callee, paren, arguments } => {
                    visitor.visit_call_expr(callee, paren, arguments)
                },
                Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
                Expr::Literal { value } => visitor.visit_literal_expr(value),
                Expr::Logical { left, operator, right } => {
                    visitor.visit_logical_expr(left, operator, right)
                },
                Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
                Expr::Variable { name } => visitor.visit_variable_expr(name),
            }
        }
    }
}
