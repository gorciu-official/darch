use std::process::Command;
use std::process;
use crate::print_error;

pub fn get_aur_packages() -> Vec<String> {
    let foreign_output = Command::new("pacman")
        .args(["-Qm"])
        .output()
        .unwrap_or_else(|_| {
            print_error("Failed to run pacman -Qm", None);
            process::exit(1);
        });

    String::from_utf8_lossy(&foreign_output.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}
