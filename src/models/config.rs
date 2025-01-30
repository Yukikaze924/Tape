use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RunConfiguration {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(rename(serialize = "pretty-json"))]
    pub pretty_json: Option<bool>
}
