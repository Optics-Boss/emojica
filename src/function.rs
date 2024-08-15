pub mod function {
    use std::{cell::RefCell, rc::Rc};

    use crate::{environment::environment::Environment, object::object::Object, stmt::stmt::Stmt, token::token::Token};


    pub enum Function {
        Native {
            arity: usize,
            body: Box<fn(&Vec<Object>) -> Object>,
        },

        User {
            name: Token,
            params: Vec<Token>,
            body: Vec<Stmt>,
            closure: Rc<RefCell<Environment>>,
            is_initializer: bool,
        },
    }
}
