#![allow(non_snake_case)]
#![allow(unused_variables)]

mod args;

use anyhow::{bail, Result};
use clap::Parser;
use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

use args::Args;

//right now, the problem with the logging function is that it just gets the file name, but it should get the path of the file as well, this can be fixed later when we add the actual del func
fn logging(){
   let args = Args::parse();   
   println!("{:?}", args); //for debugging only
   let mut log_file = OpenOptions::new().write(true).create_new(true).open("reaper_log.log").expect("Unable to open/create file"); //create the reaper_log.log file
   let result_to_log = args.files[0].as_path().display().to_string();
   let newline = "";
   let result = [result_to_log, newline.to_string()].join("\n");
   println!("{:}", result);
   //gets the file name, then file_name --> path --> string --> bytes and writes to .log
   log_file.write_all(result.as_bytes()).expect("Unable to write data");  
}



fn main(){
    let args = Args::parse();

    for file in args.files {
        logging();{
        println!("Debug: {}", Path::new(&file).exists())
        

       // if !Path::new(&file).exists() {
         //   bail!("Cannot remove '{}': no such file or directory", file.to_str().unwrap());
            

        }
    }

    //Ok(())
}
