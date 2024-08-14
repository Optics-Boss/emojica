pub mod expr {
    use crate::token::token::Token;

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
        Get {
            object: Box<Expr>,
            name: Token
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
        Set {
            object: Box<Expr>,
            name: Token,
            value: Box<Expr>
        },
        Unary {
            operator: Token,
            right: Box<Expr>
        },
        Variable {
            name: Token,
        }
    }

    pub enum LiteralValue {
        Boolean(bool),
        Null,
        Number(f64),
        String(String),
    }
}
