mod config;
mod packages;
mod printer;
mod shell;
mod users;
mod cfg;
mod system;
mod subcmds;

use clap::{Parser, Subcommand};

use crate::printer::print_error;
use crate::subcmds::add::add_package;
use crate::subcmds::rebuild::rebuild;
use crate::subcmds::remove::remove_package;
use crate::subcmds::reset_config::reset_config;

#[derive(Debug, Parser)]
#[command(name = "darch")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Rebuilds the system based on the system configuration
    Rebuild,

    /// Re-generates the system configuration
    ResetConfig,

    /// Quickly add a package 
    Add {
        name: String,
        #[arg(short, long)]
        rebuild: bool
    },
    
    /// Quickly removes a package 
    Remove {
        name: String,
        #[arg(short, long)]
        rebuild: bool
    },
}

fn main() {
    let args = Args::parse();

    let euid = unsafe { nix::libc::geteuid() };

    if euid != 0 {
        print_error(
            "darch must be run under root privileges",
            Some("current effective user ID is not 0 (root)"),
        );
        return;
    }

    match args.command {
        Commands::Rebuild => rebuild(),
        Commands::ResetConfig => {
            reset_config();
            ()
        },
        Commands::Add {name, rebuild} => add_package(name, rebuild),
        Commands::Remove {name, rebuild} => remove_package(name, rebuild),
    }
}
