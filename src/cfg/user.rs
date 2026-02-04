use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub user: String,
    pub groups: Vec<String>,
    pub displayname: Option<String>,
    pub shell: String,
}
