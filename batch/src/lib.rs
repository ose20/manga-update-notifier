use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;

pub mod model;
pub mod subcommand;

pub fn open(file_path: &str) -> Result<Box<dyn BufRead>> {
    Ok(Box::new(BufReader::new(File::open(file_path)?)))
}
