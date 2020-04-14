use std::{
    fs::File,
    io,
    io::prelude::*,
    path::{ Path, PathBuf }
};

use structopt::StructOpt;

#[derive(Debug)]
pub struct Rox;

#[derive(StructOpt, Debug)]
#[structopt(name = "rox")]
struct Args {
    #[structopt(parse(from_os_str))]
    path: Vec<PathBuf>,
}

impl Rox {
    pub fn main(args: Vec<PathBuf>) {
        if args.len() > 1 {
            panic!("Usage: jrox [script]");
        } else if args.len() == 1 {
            Rox::run_file(&args[0]); 
        } else {
            Rox::run_prompt();
        }
    }
    
    pub fn run_file<P>(path: P) -> io::Result<()> where P: AsRef<Path> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!("{:?}", file);

        Ok(())
    }

    pub fn run_prompt() {

    }

    pub fn run() {

    }
}

fn main() {
    let args = Args::from_args();
    Rox::main(args.path)
}
