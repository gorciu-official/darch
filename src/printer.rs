use colored::Colorize;

pub fn print_header(msg: &str) {
    println!("{}{}", "### ".blue(), msg.bold());
}

pub fn print_error(msg: &str, detail: Option<&str>) {
    eprintln!("{}", format!("ERROR {msg}").bright_red());
    if let Some(d) = detail {
        eprintln!("  -> {}", d);
    }
}

pub fn print_warning(msg: &str, detail: Option<&str>) {
    eprintln!("{}", format!("ERROR {msg}").bright_yellow());
    if let Some(d) = detail {
        eprintln!("  -> {}", d);
    }
}
