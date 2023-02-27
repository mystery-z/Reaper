#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod args;

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use args::Args;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;


//right now, the problem with the logging function is that it just gets the file name, but it should get the path of the file as well, this can be fixed later when we add the actual del func
fn logging(){ 
    let args = Args::parse();   
    
    //println!("{:?}", args); //for debugging only
    if !Path::new("reaper_log.log").exists(){
        let log_file = File::create("reaper_log.log"); 
    }
 
    //let log_file= File::open("reaper_log.log");
    let mut log_file = OpenOptions::new().write(false).append(true).create(false).open("reaper_log.log").expect("Unable to open/create file"); //create the reaper_log.log file
    let result_to_log = args.files[0].as_path().display().to_string();
    let newline = "";
    let result = [result_to_log, newline.to_string()].join("\n");
    //println!("{:}", result); //for debugging only
    //gets the file name, then file_name --> path --> string --> bytes and writes to .log
    log_file.write_all(result.as_bytes()).expect("Unable to write data");
}



fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if !Path::new("/tmp/grave").exists() {
        if let Err(_err) = fs::create_dir("/tmp/grave") {
            bail!("reap: Failed to create grave directory: {}", _err)
        }
    }

    for file in &args.files {
        
        if !Path::new(&file).exists() {
            println!("reap: Cannot remove {:?}: no such file or directory", file);
            continue;
        }
       // logging();
        let absolute_path = fs::canonicalize(file).unwrap();

        let mut grave = PathBuf::from("/tmp/grave");
        grave = grave.join(&absolute_path.strip_prefix("/").unwrap());

        if let Err(_err) = fs::create_dir_all(&grave.parent().unwrap()) {
            bail!("Failed to create directory {}: {}", &grave.display(), _err);
        }

        if let Err(_err) = fs::rename(&absolute_path, &grave) {
            bail!(
                "Failed to move file from {} to {}: {}",
                &absolute_path.display(),
                &grave.display(),
                _err
            )
        };
        logging();
    }


    Ok(())
}
