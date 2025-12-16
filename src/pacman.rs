use crate::printer::print_error;
use crate::shell::run;
use std::process;
use std::process::Command;

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

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(|s| s.to_string())
        .collect()
}
