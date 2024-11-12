use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RunConfiguration {
    #[serde(default)]
    pub debug: Option<bool>,
    #[serde(default)]
    pub format: Option<String>
}