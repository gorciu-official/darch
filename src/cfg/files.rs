use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileConfig {
    pub path: String,
    pub source: Option<String>,
    pub content: Option<String>,
    pub mode: Option<String>,
}
