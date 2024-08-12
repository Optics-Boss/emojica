use std::{env, fs};
use std::process::ExitCode;
use std::io::{stdin};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        return ExitCode::from(64)
    } else if args.len() == 2 {
        let query = &args[1];
        run_file(query.to_string())
    } else {
        run_prompt()
    }

    ExitCode::SUCCESS
}


fn run_file(path: String){
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
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
