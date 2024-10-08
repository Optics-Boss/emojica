pub mod function {
    use core::fmt;
    use std::{cell::RefCell, rc::Rc};

    use crate::{environment::environment::Environment, interpreter::interpreter::Interpreter, object::object::Object, parser::parser::Error, stmt::stmt::Stmt, token::token::Token};


    #[derive(Clone, Debug)]
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

    impl Function {
        pub fn call(
            &self,
            interpreter: &mut Interpreter,
            arguments: &Vec<Object>,
        ) -> Result<Object, Error> {
            match self {
                Function::Native { body, .. } => Ok(body(arguments)),
                Function::User {
                    params,
                    body,
                    closure,
                    is_initializer,
                    ..
                } => {
                    let environment = Rc::new(RefCell::new(Environment::from(closure)));
                    for (param, argument) in params.iter().zip(arguments.iter()) {
                        environment
                            .borrow_mut()
                            .define(param.lexeme.clone(), argument.clone());
                    }
                    match interpreter.execute_block(body, environment) {
                        Err(Error::Return { value }) => {
                            if *is_initializer {
                                Ok(closure
                                    .borrow()
                                    .get_at(0, "this")
                                    .expect("Initializer should return 'this'."))
                            } else {
                                Ok(value)
                            }
                        }
                        Err(other) => Err(other),
                        Ok(..) => {
                            if *is_initializer {
                                Ok(closure
                                    .borrow()
                                    .get_at(0, "this")
                                    .expect("Initializer should return 'this'."))
                            } else {
                                Ok(Object::Null)
                            }
                        }
                    }
                }
            }
        }

        pub fn bind(&self, instance: Object) -> Self {
            match self {
                Function::Native { .. } => unreachable!(),
                Function::User {
                    name,
                    params,
                    body,
                    closure,
                    is_initializer,
                } => {
                    let environment = Rc::new(RefCell::new(Environment::from(closure)));
                    environment
                        .borrow_mut()
                        .define("this".to_string(), instance);
                    Function::User {
                        name: name.clone(),
                        params: params.clone(),
                        body: body.clone(),
                        closure: environment,
                        is_initializer: *is_initializer,
                    }
                }
            }
        }

        pub fn arity(&self) -> usize {
            match self {
                Function::Native { arity, .. } => *arity,
                Function::User { params, .. } => params.len(),
            }
        }
    }

    impl fmt::Display for Function {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Function::Native { .. } => write!(f, "<native func>"),
                Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
            }
        }
    }
}
