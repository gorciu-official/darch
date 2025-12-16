use crate::printer::{print_error, print_warning};
use std::fs;
use std::process::Command;

pub fn run(cmd: &str, args: &[&str]) -> bool {
    let status = Command::new(cmd).args(args).status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            print_error("Failed to run command", Some(&e.to_string()));

            false
        }
    }
}

pub fn check_for_shell_warnings() {
    let users_file = fs::read_to_string("/etc/passwd").unwrap_or_default();
    let shell_file = fs::read_to_string("/etc/shells").unwrap_or_default();

    let shells: Vec<&str> = shell_file
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .collect();

    for user in users_file.lines() {
        let parts: Vec<&str> = user.split(':').collect();

        if parts.len() < 7 {
            continue;
        }

        let username = parts[0];
        let displayname = parts[4];
        let shell = parts[6];

        if shell == "nologin" || shell == "/bin/nologin" || shell == "/usr/bin/nologin" {
            continue;
        }

        if !shells.contains(&shell) {
            print_warning(
                &format!(
                    "Applying this system configuration breaks shell for {} ({})",
                    if !displayname.trim().is_empty() {
                        displayname
                    } else {
                        username
                    },
                    username
                ),
                Some(&format!("Shell {} is not in /etc/shells", shell)),
            );
        }
    }
}
