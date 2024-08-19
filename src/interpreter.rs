pub mod interpreter {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use crate::{environment::environment::Environment, expr::expr::Expr, object::object::Object, parser::parser::Error, stmt::stmt::Stmt, token::token::Token};

    pub struct Interpreter {
        pub globals: Rc<RefCell<Environment>>,
        environment: Rc<RefCell<Environment>>,
        locals: HashMap<Token, usize>,
    }

    impl Interpreter {
        pub fn new() -> Self {
            let globals = Rc::new(RefCell::new(Environment::new()));
            Interpreter {
                globals: Rc::clone(&globals),
                environment: Rc::clone(&globals),
                locals: HashMap::new(),
            }
        }

        pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
            for statement in statements {
                self.execute(statement)?;
            }

            Ok(())
        }

        fn evaluate(&mut self, expression: &Expr) -> Result<Object, Error> {
            expression.accept(self)
        }

        fn execute(&mut self, statement: &Stmt) -> Result<(), Error> {
            statement.accept(self)
        }

        pub fn resolve(&mut self, name: &Token, depth: usize) {
            self.locals.insert(name.clone(), depth);
        }

    }
}
