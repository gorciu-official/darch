use std::collections::HashMap;
use std::fs;
use crate::config::UserConfig;

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

        if uid < 1000 {
            continue;
        }

        let displayname = parts[4].to_string();
        let shell = parts[6].to_string();

        if shell.contains("nologin") {
            continue;
        }

        let groups = groups_map
            .get(&user)
            .cloned()
            .unwrap_or_default();

        users.push(UserConfig {
            user,
            groups,
            displayname,
            shell,
        });
    }

    users
}
