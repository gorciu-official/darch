use crate::printer::print_error;
use crate::shell::run;
use std::process;
use std::process::Command;
use std::collections::HashSet;

pub fn is_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Qi", pkg])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn install_if_missing(packages: &[String]) -> bool {
    let missing: Vec<&String> = packages.iter().filter(|p| !is_installed(p)).collect();

    if missing.is_empty() {
        println!("added packages already installed before by pacman");

        return true;
    }

    let args: Vec<&str> = ["-S", "--color", "never"]
        .into_iter()
        .chain(missing.iter().map(|s| s.as_str()))
        .collect();

    run("/usr/bin/pacman", &args)
}

pub fn get_explicit_packages() -> Vec<String> {
    let output = Command::new("pacman")
        .args(["-Qe"])
        .output()
        .unwrap_or_else(|_| {
            print_error("Failed to run pacman -Qe", None);
            process::exit(1);
        });

    let foreign_output = Command::new("pacman")
        .args(["-Qm"])
        .output()
        .unwrap_or_else(|_| {
            print_error("Failed to run pacman -Qm", None);
            process::exit(1);
        });

    let foreign: HashSet<String> = String::from_utf8_lossy(&foreign_output.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_owned)
        .collect();

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .filter(|pkg| !foreign.contains(*pkg))
        .map(str::to_owned)
        .collect()
}
