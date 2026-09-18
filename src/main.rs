mod config;
mod pacman;
mod printer;
mod shell;
mod users;
mod cfg;
mod system;
mod subcmds;

use clap::{Parser, ValueEnum};

use crate::printer::print_error;

use crate::subcmds::rebuild::rebuild;
use crate::subcmds::reset_config::reset_config;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum Mode {
    /// rebuilds the system based on the system configuration
    Rebuild,
    /// re-autogenerates the system configuration
    ResetConfig 
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(help = "What darch should do?")]
    mode: Mode
}

fn main() {
    let args = Args::parse();
    let euid = unsafe { 
        nix::libc::geteuid()
    };

    if euid != 0 {
        print_error("darch must be run under root privileges", Some("current effective user ID is not 0 (root)"));
        return;
    }

    if args.mode == Mode::Rebuild {
        rebuild();
    } else if args.mode == Mode::ResetConfig {
        reset_config(); 
    }
}
