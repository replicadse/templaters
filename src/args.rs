use std::{
    error::Error,
    result::Result,
};

use crate::error::UnknownCommandError;

#[derive(Debug)]
/// Combined arguments struct for the invoked command incl. all necessary
/// information.
pub struct CallArgs {
    /// The privilege with which the program was called.
    pub privilege: Privilege,
    /// The subcommand that was called incl. all arguments and parameters.
    pub command: Command,
}

impl CallArgs {
    /// Validating the arguments since some commands may only be called with
    /// certain privileges, arguments being XOR or similar.
    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        match self.privilege {
            | Privilege::Normal => Ok(()),
            | Privilege::Experimental => Ok(()),
        }
    }
}

#[derive(Debug)]
/// The privilege.
pub enum Privilege {
    /// Normal privileges identify the normal scenario.
    Normal,
    /// Experimental privileges give access to unstable features.
    Experimental,
}

#[derive(Debug)]
/// The (sub-)command representation for the call args.
pub enum Command {
    /// Sample subcommand representation.
    Sample,
}

/// The type that parses the arguments to the program.
pub struct ClapArgumentLoader {}

impl ClapArgumentLoader {
    /// Parsing the program arguments with the `clap` trait.
    pub fn load() -> Result<CallArgs, Box<dyn Error>> {
        let command = clap::App::new("templaters")
            .version(env!("CARGO_PKG_VERSION"))
            .about("templaters")
            .author("Weber, Heiko Alexander <haw@voidpointergroup.com>")
            .arg(
                clap::Arg::with_name("experimental")
                    .short("e")
                    .long("experimental")
                    .value_name("EXPERIMENTAL")
                    .help("Enables experimental features that do not count as stable.")
                    .required(false)
                    .takes_value(false),
            )
            .subcommand(clap::App::new("sample").about("sample"))
            .get_matches();

        let privilege = if command.is_present("experimental") {
            Privilege::Experimental
        } else {
            Privilege::Normal
        };

        let cmd = if let Some(_) = command.subcommand_matches("collect") {
            Command::Sample
        } else {
            return Err(Box::new(UnknownCommandError::new("unknown command")));
        };

        let callargs = CallArgs {
            privilege,
            command: cmd,
        };

        callargs.validate()?;
        Ok(callargs)
    }
}
