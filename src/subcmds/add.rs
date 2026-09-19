use std::fs;

use crate::{config::SysConfig, printer::{print_error, print_header}, rebuild};

pub fn add_package(name: String, do_rebuild: bool) {
    print_header("Modifing system configuration");
    let mut cfg = SysConfig::read_or_generate_config("/etc/sysconfig");

    if cfg.packages.as_ref().is_some_and(|packages| packages.contains(&name)) {
        println!("package {name} is already installed");
        return;
    }

    println!("adding package {name}");
    cfg.packages.get_or_insert_with(Vec::new).push(name);

    println!("writting new configuration");
    let dump = toml::to_string_pretty(&cfg).unwrap(); // TODO: error handling
    if let Err(e) = fs::write("/etc/sysconfig", &dump) {
        print_error("could not write new configuration", Some(&e.to_string()));
    }

    if do_rebuild { 
        rebuild();
    }
}
