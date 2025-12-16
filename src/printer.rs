pub fn print_header(msg: &str) {
    println!("\x1b[34m### \x1b[0;1m{}\x1b[0m", msg);
}

pub fn print_error(msg: &str, detail: Option<&str>) {
    eprintln!("[\x1b[91m ERROR \x1b[0m] {}", msg);
    if let Some(d) = detail {
        eprintln!("  -> {}", d);
    }
}

pub fn print_warning(msg: &str, detail: Option<&str>) {
    eprintln!("[\x1b[93m WARN \x1b[0m] {}", msg);
    if let Some(d) = detail {
        eprintln!("  -> {}", d);
    }
}
