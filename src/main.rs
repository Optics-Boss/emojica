use std::{env, fs, io };
use std::process::{exit, ExitCode};
use std::io::{stdin, BufRead};

use interpreter::interpreter::Interpreter;
use parser::parser::{Error, Parser};
use resolver::resolver::Resolver;
use scanner::scanner::Scanner;
use token::token::{Token, TokenType};

pub mod token;
pub mod scanner;
pub mod parser;
pub mod stmt;
pub mod expr;
pub mod interpreter;
pub mod environment;
pub mod object;
pub mod function;
pub mod resolver;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    let mut emojica = Emojica::new();
    match args.as_slice() {
        [_, file] => match emojica.run_file(file) {
            Ok(_) => (),
            Err(Error::Return { .. }) => unreachable!(),
            Err(Error::Runtime { message, .. }) => {
                eprintln!("Error: {}", message);
                exit(70)
            }
            Err(Error::Parse) => exit(65),
            Err(Error::Io(_)) => unimplemented!(),
        },
        [_] => emojica.run_prompt()?,
        _ => {
            eprintln!("Usage: lox-rs [script]");
            exit(64)
        }
    }
    Ok(())
}

struct Emojica {
    interpreter: Interpreter,
}

impl Emojica {
    fn new() -> Self {
        Emojica {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path);
        self.run(source?)
    }

    fn run_prompt(&mut self) -> Result<(), Error> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.run(line?);
            print!("> ");
        }
        Ok(())
    }

  fn run(&mut self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens.to_vec());
        let statements = parser.parse()?;

        let mut resolver = Resolver::new(&mut self.interpreter);
        resolver.resolve_stmts(&statements);

        if resolver.had_error {
            return Ok(());
        }

        self.interpreter.interpret(&statements)?;
        Ok(())
    }

}


fn error(line: i32, message: String) {
   report(line, String::from(""), message);
}

fn report(line: i32, where_report: String, message: String) {
    eprintln!("[line {:?}] Error {:?} : {:?}", line, where_report, message);
}

fn parser_error(token: &Token, message: String) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end".to_string(), message);
    } else {
        report(token.line, " at end".to_string(), message);
    }
}
