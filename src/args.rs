use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "reap", author, version, about, long_about = None)]
pub struct Args {
    /// Enter file name to delete
    #[arg(value_name = "FILE", value_delimiter = ' ')]
    pub file_name: Vec<PathBuf>,
}
