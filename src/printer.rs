use colored::Colorize;
use std::io::{self, Write};

pub fn print_header(msg: &str) {
    println!("{}{}", "### ".blue(), msg.bold());
}

pub fn print_error(msg: &str, detail: Option<&str>) {
    eprintln!("{} {}", "error:".bright_red().bold(), msg);

    if let Some(d) = detail {
        eprintln!("   {}", d);
    }
}

pub fn print_warning(msg: &str, detail: Option<&str>) {
    eprintln!("{} {}", "warning:".bright_yellow().bold(), msg);

    if let Some(d) = detail {
        eprintln!("   {}", d);
    }
}

pub fn ask_yes_no(prompt: &str) -> bool {
    print_warning(prompt, None);
    print!("Would you like to continue? [y/N] ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    matches!(input.trim(), "y" | "Y")
}
