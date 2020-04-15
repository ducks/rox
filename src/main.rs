use std::{
    fs::File,
    io,
    io::prelude::*,
    path::{ Path, PathBuf }
};

mod scanner;
mod token;

use scanner::Scanner;
use token::Token;


use structopt::StructOpt;

#[derive(Debug, Default)]
struct Rox {
    has_error: bool,
    is_at_end: bool
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rox")]
struct Args {
    #[structopt(parse(from_os_str))]
    path: Vec<PathBuf>,
}

impl Rox {
    pub fn new() -> Self {
        Rox {
            ..Default::default()
        }
    }

    pub fn main(args: Vec<PathBuf>) {
        if args.len() > 1 {
            panic!("Usage: rox [script]");
        } else if args.len() == 1 {
            Rox::run_file(&args[0]); 
        } else {
            Rox::run_prompt();
        }
    }
    
    pub fn run_file<P>(path: P) -> io::Result<()> 
        where P: AsRef<Path> {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            Rox::run(&contents);

            Ok(())
    }

    pub fn run_prompt() -> io::Result<()> {
        let mut input = String::new();

        loop {
            print!("> ");
            let stdin = io::stdin();
            io::stdout().flush();
            stdin.read_line(&mut input)?;
            Rox::run(&input);
        }

        Ok(())
    }

    pub fn run(contents: &String) {
        let mut scanner = Scanner::new(contents);

        let tokens: &Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:#?}", token);
        }
    }

    pub fn error(line: i32, message: String) {
        Rox::report(line, message)
    }

    fn report(line: i32, message: String) {
        eprintln!("[line {} + ] Error: {}", line, message);
    }
}

fn main() {
    let args = Args::from_args();
    let rox = Rox::new();
    println!("{:#?}", rox);

    rox::main(args.path)
}
