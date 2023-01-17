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
    for i in traverse_dir(dir, &regex) {
        println!("{}", i);
    }

    Ok(())
}

fn traverse_dir(dir: ReadDir, regex: &Regex) -> Vec<String> {
    let mut result = Vec::new();
    
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
                                    result.append(&mut traverse_dir(new_dir, &regex))
                                },
                                Err(err) => println!("Failed to open directory {:?} with following error: {}, skipping.", entry.path(), err),
                            }
                        } else {
                            if let Some(path) = entry.path().to_str() {
                                if regex.is_match(path) {
                                    result.push(path.to_string());
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

    result
}