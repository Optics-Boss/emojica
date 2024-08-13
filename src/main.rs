use std::{env, fs};
use std::process::ExitCode;
use std::io::stdin;

use token::token::{Token, TokenType};

pub mod token;
pub mod scanner;
pub mod parser;
pub mod stmt;
pub mod expr;

static ERROR_STATE : bool = false;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        return ExitCode::from(64)
    } else if args.len() == 2 {
        let query = &args[1];
        run_file(query.to_string());
    } else {
        run_prompt()
    }

    ExitCode::SUCCESS
}


fn run_file(path: String) -> ExitCode {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    if ERROR_STATE {
        return ExitCode::from(65);
    }

    ExitCode::SUCCESS
}

fn run_prompt(){
    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        run(buffer)
    }
}

fn run(source: String) {

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
        report(token.line, " at end", message);
    }
}
