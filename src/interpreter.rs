pub mod interpreter {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use crate::{environment::environment::Environment, token::token::Token};

    pub struct Interpreter {
        pub globals: Rc<RefCell<Environment>>,
        environment: Rc<RefCell<Environment>>,
        locals: HashMap<Token, usize>,
    }
}
