#![allow(non_snake_case)]
#![allow(unused_variables)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "reap", author, version, about, long_about = None)]
struct Args {
    /// Enter file name to delete
    #[arg(short, long, value_name = "FILE")]
    file_name: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("Debug: {}", args.file_name.unwrap().into_os_string().into_string().unwrap())
}
