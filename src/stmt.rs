pub mod stmt {
    use crate::{expr::expr::Expr, token::token::Token};

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
}
