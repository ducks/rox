use std::{
    fs::File,
    io,
    io::prelude::*,
    path::Path,
};

mod scanner;
mod token;

use scanner::Scanner;
use token::Token;

#[derive(Debug, Default)]
pub struct Rox {
    has_error: bool,
    is_at_end: bool
}

impl Rox {
    pub fn new() -> Self {
        Rox {
            ..Default::default()
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

    pub fn run<T: std::default::Default>(contents: &String) {
        let mut scanner = Scanner::<T>::new(&contents);
    }

    pub fn error(line: i32, message: String) {
        Rox::report(line, message)
    }

    fn report(line: i32, message: String) {
        eprintln!("[line {} + ] Error: {}", line, message);
    }
}
