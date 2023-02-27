#![allow(non_snake_case)]
#![allow(unused_variables)]

mod args;

use clap::Parser;
use std::path::Path;

use args::Args;


fn main() {
    let args = Args::parse();

    for file in args.file_name {
        println!("Debug: {}", Path::new(&file).exists())
    }
}
