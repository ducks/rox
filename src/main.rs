use std::{
    fs::File,
    io,
    io::prelude::*,
    path::{ Path, PathBuf }
};

use structopt::StructOpt;

mod rox;

use rox::Rox;

#[derive(StructOpt, Debug)]
#[structopt(name = "rox")]
struct Args {
    #[structopt(parse(from_os_str))]
    path: Vec<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    if args.path.len() > 1 {
        panic!("Usage: rox [script]");
    } else if args.path.len() == 1 {
        Rox::run_file(&args.path[0]); 
    } else {
        Rox::run_prompt();
    }
}
