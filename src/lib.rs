use std::{error::Error, fs::{read_dir, ReadDir}};

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version )]
#[command(about = "GNU find clone made in rust as a programming exercise.")]
#[command(long_about = "GNU find clone made in rust. This app is not meant for actual use, and was made as an exercise in programming.")]
pub struct AppArgs {
    /// A path from which the search will start
    start_path: String,
    /// Only paths that match this regex will be returned
    regex: String
}

pub fn run(args: AppArgs) -> Result<(), Box<dyn Error>> {
    let dir = read_dir(args.start_path)?;
    let regex = Regex::new(&args.regex)?;
    
    traverse_dir(dir, &regex);

    Ok(())
}

/// Recursively goes through the directory and prints all directories /files
/// that match the supplied regex.
fn traverse_dir(dir: ReadDir, regex: &Regex) {
    // Go through the directory contents
    for i in dir {
        match i {
            Ok(entry) => {
                match entry.file_type() {
                    Ok(file_type) => {
                        // If the file is a directory, we recursively go there too
                        if file_type.is_dir() {
                            match read_dir(entry.path()) {
                                Ok(new_dir) => {
                                    traverse_dir(new_dir, &regex)
                                },
                                Err(err) => println!("Failed to open directory {:?} with following error: {}, skipping.", entry.path(), err),
                            }
                        } else {
                            if let Some(path) = entry.path().to_str() {
                                if regex.is_match(path) {
                                    println!("{}", path.to_string());
                                }
                            }
                        }
                    },
                    Err(err) => println!("Failed to get file type with following error: {:?}, skipping.", err),
                }
            },
            Err(err) => {
                println!("Failed to read directory entry with following error: {:?}, skipping.", err);
            },
        }
    }
}