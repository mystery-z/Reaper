#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#![allow(unused_variables)]

mod args;

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

use args::Args;
use std::fs::{File, OpenOptions};
use std::io::Write;

//right now, the problem with the logging function is that it just gets the file name, but it should get the path of the file as well, this can be fixed later when we add the actual del func
fn logging(grave_path: PathBuf, original_path: PathBuf) -> Result<(), anyhow::Error> {
    if !Path::new("/tmp/grave/.log").exists() {
        if let Err(_err) = fs::File::create("/tmp/grave/.log") {
            bail!("reap: Failed to create grave directory: {}", _err);
        }
    }

    let mut log_file = OpenOptions::new()
        .write(false)
        .append(true)
        .create(false)
        .open("/tmp/grave/.log")
        .expect("Unable to open file");

    let mut log_data = String::from(grave_path.display().to_string()) + " ";
    log_data = log_data + &String::from(original_path.display().to_string()) + "\n";

    log_file
        .write_all(log_data.as_bytes())
        .expect("reap: Unable to write data to log");

    Ok(())
}

// TODO: Make a check to see if file with same name already exists. Do the same for when undo
// command is run.

fn delete_files(files: &Vec<PathBuf>) -> Result<(), anyhow::Error> {
    for file in files {
        if !Path::new(&file).exists() {
            println!("reap: Cannot remove {:?}: no such file or directory", file);
            continue;
        }
        let absolute_path = fs::canonicalize(file).unwrap();

        let mut grave = PathBuf::from("/tmp/grave");
        grave = grave.join(&absolute_path.strip_prefix("/").unwrap());

        if let Err(_err) = fs::create_dir_all(&grave.parent().unwrap()) {
            bail!(
                "reap: Failed to create directory {}: {}",
                &grave.display(),
                _err
            );
        }

        if let Err(_err) = fs::rename(&absolute_path, &grave) {
            bail!(
                "reap: Failed to move file from {} to {}: {}",
                &absolute_path.display(),
                &grave.display(),
                _err
            )
        };

        logging(grave, absolute_path);
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if !Path::new("/tmp/grave").exists() {
        if let Err(_err) = fs::create_dir("/tmp/grave") {
            bail!("reap: Failed to create grave directory: {}", _err)
        }
    }

    delete_files(&args.files);

    Ok(())
}
