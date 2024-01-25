use anyhow::Result;
use std::error::Error;
use structopt::StructOpt;
use cli::CliCommand;

mod api;
mod config;
mod merkle;
mod utils;
mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = CliCommand::from_args();
    cmd.execute()?;
    Ok(())
}

