use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "RmX", author, version, about, long_about = None)]
pub struct Args {
    /// Enter file name to delete
    #[arg(value_name = "FILE", value_delimiter = ' ')]
    pub files: Vec<PathBuf>,
    
    /// Enter the file to undo
    #[arg(short='u', value_name = "FILE", value_delimiter = ' ')]
    pub undo_file: Vec<PathBuf>
    

}
