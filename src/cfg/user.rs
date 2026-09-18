use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPackages {
    pub flatpak: Option<Vec<String>>,
    pub aur: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub user: String,
    pub groups: Vec<String>,
    pub displayname: Option<String>,
    pub shell: String,
    pub packages: UserPackages
}
