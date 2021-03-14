#![deny(missing_docs)]

//! templaters

use std::error::Error;

mod args;
mod commands;
mod error;

use crate::{
    args::{
        ClapArgumentLoader,
        Command,
    },
    commands::sample,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = ClapArgumentLoader::load()?;
    match args.command {
        | Command::Sample => {
            sample()?;
            Ok(())
        },
    }
}
