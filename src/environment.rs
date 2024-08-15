pub mod environment {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use crate::object::object::Object;


    pub struct Environment {
        pub enclosing: Option<Rc<RefCell<Environment>>>,
        values: HashMap<String, Object>
    }
}
