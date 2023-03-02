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
use std::time::{SystemTime, UNIX_EPOCH};

use args::Args;
use std::fs::{File, OpenOptions};
use std::io::Write;

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
        .expect("Unable to open log file");

    let log_data = String::from(grave_path.display().to_string())
        + " "
        + &String::from(original_path.display().to_string())
        + "\n";

    log_file
        .write_all(log_data.as_bytes())
        .expect("reap: Unable to write data to log");

    Ok(())
}

fn undo() {}

// TODO: Make a check to see if file with same name already exists. Do the same for when undo
// command is run.

fn delete_files(files: &Vec<PathBuf>, UNIX_TIME: &u64) -> Result<(), anyhow::Error> {
    for file in files {
        if !Path::new(&file).exists() {
            println!("reap: Cannot remove {:?}: no such file or directory", file);
            continue;
        }

        let absolute_path = fs::canonicalize(file).unwrap();

        let mut grave = PathBuf::from("/tmp/grave");
        grave = grave.join(&absolute_path.strip_prefix("/").unwrap());


        if let Some(extention) = grave.extension() {
            let grave_extention = grave.extension().unwrap().to_str().unwrap();
            let grave_stem = grave.file_stem().unwrap().to_str().unwrap();

            let mut grave_path = format!("{}-{}", grave_stem, UNIX_TIME);
            grave_path = format!("{}.{}", grave_path, grave_extention);

            grave.set_file_name(grave_path);
        } 
        else {
            grave.set_file_name(format!("{}-{}", grave.file_stem().unwrap().to_str().unwrap(), UNIX_TIME));

        }



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
    let UNIX_TIME = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("{}", UNIX_TIME);

    let args = Args::parse();

    if !Path::new("/tmp/grave").exists() {
        if let Err(_err) = fs::create_dir("/tmp/grave") {
            bail!("reap: Failed to create grave directory: {}", _err)
        }
    }

    delete_files(&args.files, &UNIX_TIME);

    Ok(())
}
