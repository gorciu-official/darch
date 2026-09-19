use std::fs;

use crate::{config::SysConfig, printer::{print_error, print_header}, subcmds::rebuild::rebuild};

pub fn remove_package(name: String, do_rebuild: bool) {
    print_header("Modifying system configuration");
    let mut cfg = SysConfig::read_or_generate_config("/etc/sysconfig");

    let Some(packages) = cfg.packages.as_mut() else {
        print_error("package is not in configuration", None);
        return;
    };

    if !packages.contains(&name) {
        print_error("package is not in configuration", None);
        return;
    }

    println!("removing package {name}");

    packages.retain(|package| package != &name);

    println!("writing new configuration");

    let dump = toml::to_string_pretty(&cfg).unwrap(); // TODO: error handling

    if let Err(e) = fs::write("/etc/sysconfig", &dump) {
        print_error("could not write new configuration", Some(&e.to_string()));
        return;
    }

    if do_rebuild {
        rebuild();
    }
}
