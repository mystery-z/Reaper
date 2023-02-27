#![allow(non_snake_case)]
#![allow(unused_variables)]

mod args;

use anyhow::{bail, Result};
use clap::Parser;
use std::path::Path;

use args::Args;

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    for file in args.files {
        if !Path::new(&file).exists() {
            bail!("Cannot remove '{}': no such file or directory", file.to_str().unwrap());
        }
    }

    Ok(())
}
