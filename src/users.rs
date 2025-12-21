use crate::cfg::user::UserConfig;
use crate::config::SysConfig;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

pub fn ensure_groups_exist(groups: &[String]) {
    for group in groups {
        let exists = Command::new("getent")
            .arg("group")
            .arg(group)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !exists {
            let status = Command::new("groupadd").arg(group).status();
            if status.is_err() || !status.unwrap().success() {
                eprintln!("failed to create group {}", group);
            }
        }
    }
}

pub fn users_to_map(cfg: &SysConfig) -> HashMap<String, &UserConfig> {
    cfg.users
        .as_ref()
        .into_iter()
        .flat_map(|v| v.iter())
        .map(|u| (u.user.as_str().to_string(), u))
        .collect()
}

pub fn get_user_groups() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    let content = fs::read_to_string("/etc/group").unwrap_or_default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            continue;
        }

        let group = parts[0];
        let users = parts[3];

        for user in users.split(',').filter(|u| !u.is_empty()) {
            map.entry(user.to_string())
                .or_insert_with(Vec::new)
                .push(group.to_string());
        }
    }

    map
}

fn get_primary_group(gid: &str) -> Option<String> {
    let content = fs::read_to_string("/etc/group").unwrap_or_default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 3 {
            continue;
        }

        if parts[2] == gid {
            return Some(parts[0].to_string());
        }
    }

    None
}

pub fn create_user(user: &UserConfig) {
    ensure_groups_exist([user.user.clone()].as_ref());

    let groups = user
        .groups
        .iter()
        .filter(|g| *g != &user.user)
        .cloned()
        .collect::<Vec<_>>();
    ensure_groups_exist(&groups);

    let mut cmd = Command::new("useradd");
    cmd.arg("-m")
        .arg("-d")
        .arg(format!("/home/{}", user.user))
        .arg("-g")
        .arg(&user.user)
        .arg("-s")
        .arg(&user.shell);

    if !groups.is_empty() {
        cmd.arg("-G").arg(groups.join(","));
    }

    if let Some(ref displayname) = user.displayname {
        cmd.arg("-c").arg(displayname);
    }

    cmd.arg(&user.user);

    let status = cmd.status();
    if status.is_err() || !status.unwrap().success() {
        eprintln!("failed to create user {}", user.user);
    }
}

pub fn get_users() -> Vec<UserConfig> {
    let groups_map = get_user_groups();
    let mut users = Vec::new();

    let content = fs::read_to_string("/etc/passwd").unwrap_or_default();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 7 {
            continue;
        }

        let user = parts[0].to_string();
        let uid: u32 = parts[2].parse().unwrap_or(0);
        let gid = parts[3];

        if uid < 1000 {
            continue;
        }

        let shell = parts[6].trim().to_string();
        if shell.contains("nologin") {
            continue;
        }

        let raw_gecos = parts[4].split(',').next().unwrap_or("").trim();
        let displayname = if raw_gecos.is_empty() {
            None
        } else {
            Some(raw_gecos.to_string())
        };

        let mut groups = groups_map.get(&user).cloned().unwrap_or_default();

        if let Some(primary) = get_primary_group(gid) {
            groups.push(primary);
        }

        groups.sort();
        groups.dedup();

        users.push(UserConfig {
            user,
            groups,
            displayname,
            shell,
        });
    }

    users
}
