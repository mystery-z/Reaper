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
    }

    Ok(())
}
