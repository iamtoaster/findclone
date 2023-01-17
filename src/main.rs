use std::error::Error;

use clap::Parser;
use findclone::{run, AppArgs};

fn main() -> Result<(), Box<dyn Error>> {
    let args = AppArgs::parse();

    run(args)
}
