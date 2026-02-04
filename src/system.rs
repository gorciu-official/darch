use crate::printer::print_error;

use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

use std::fs;
use std::path::Path;

use crate::printer::{print_header};
use crate::shell::run;
use crate::config::{SysConfig};
use crate::cfg::system::{SystemConfig};

fn apply_timezone(cfg: &SystemConfig, prev: Option<&SystemConfig>) {
    let tz = match &cfg.timezone {
        Some(t) => t,
        None => return,
    };

    if prev.and_then(|p| p.timezone.as_ref()) == Some(tz) {
        return;
    }

    let zone_path = format!("/usr/share/zoneinfo/{}", tz);

    if !Path::new(&zone_path).exists() {
        print_error("Invalid timezone", Some(tz));
        return;
    }

    print_header("setting timezone");
    run("/usr/bin/timedatectl", &["set-timezone", tz]);
}

fn apply_locale(cfg: &SystemConfig, prev: Option<&SystemConfig>) {
    let locale = match &cfg.locale {
        Some(l) => l,
        None => return,
    };

    if prev.and_then(|p| p.locale.as_ref()) == Some(locale) {
        return;
    }

    print_header("Setting locale");

    let locale_gen_path = "/etc/locale.gen";
    let locale_conf_path = "/etc/locale.conf";

    if Path::new(locale_gen_path).exists() {
        let file = fs::File::open(locale_gen_path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            let mut lines: Vec<String> = Vec::new();

            for line in reader.lines().flatten() {
                if line.trim_start().starts_with(&format!("#{}", locale)) {
                    lines.push(locale.to_string());
                } else {
                    lines.push(line);
                }
            }

            if let Ok(mut f) = fs::File::create(locale_gen_path) {
                for l in lines {
                    writeln!(f, "{}", l).ok();
                }
            }
        }
    } else {
        print_error("locale.gen not found, skipping", Some(locale_gen_path));
    }

    run("/usr/bin/locale-gen", &[]);

    if let Err(e) = fs::write(locale_conf_path, format!("LANG={}\n", locale)) {
        print_error("Failed to write /etc/locale.conf", Some(&e.to_string()));
    }
}

fn apply_keymap(cfg: &SystemConfig, prev: Option<&SystemConfig>) {
    let keymap = match &cfg.keymap {
        Some(k) => k,
        None => return,
    };

    if prev.and_then(|p| p.keymap.as_ref()) == Some(keymap) {
        return;
    }

    print_header("setting keymap");

    let content = format!("KEYMAP={}\n", keymap);

    if let Err(e) = fs::write("/etc/vconsole.conf", content) {
        print_error("Failed to write /etc/vconsole.conf", Some(&e.to_string()));
        return;
    }

    run("/usr/bin/localectl", &["set-keymap", keymap]);
}

fn apply_hostname(cfg: &SystemConfig, prev: Option<&SystemConfig>) {
    let hostname = match &cfg.hostname {
        Some(h) => h,
        None => return,
    };

    if prev.and_then(|p| p.hostname.as_ref()) == Some(hostname) {
        return;
    }

    print_header("setting hostname");

    if let Err(e) = fs::write("/etc/hostname", hostname) {
        print_error("Failed to write /etc/hostname", Some(&e.to_string()));
        return;
    }

    run("/usr/bin/hostnamectl", &["set-hostname", hostname]);
}

pub fn apply_system_config(cfg: &SysConfig, prev: &SysConfig) {
    let cfg_sys = match &cfg.system {
        Some(s) => s,
        None => return,
    };

    let prev_sys = prev.system.as_ref();

    apply_hostname(cfg_sys, prev_sys);
    apply_timezone(cfg_sys, prev_sys);
    apply_locale(cfg_sys, prev_sys);
    apply_keymap(cfg_sys, prev_sys);
}
