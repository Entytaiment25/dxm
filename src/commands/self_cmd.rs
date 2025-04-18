//! Contains commands to manage the dxm installation.

use std::error::Error;

use clap::{ArgMatches, Command};

pub mod setup;
pub mod uninstall;
pub mod update;

/// The command structure.
pub fn cli() -> Command {
    Command::new("self")
        .about("Manage the dxm installation")
        .subcommand(setup::cli())
        .subcommand(uninstall::cli())
        .subcommand(update::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match args.subcommand() {
        Some(("setup", m)) => setup::execute(m)?,
        Some(("uninstall", m)) => uninstall::execute(m)?,
        Some(("update", m)) => update::execute(m)?,
        _ => unreachable!(),
    }

    Ok(())
}
