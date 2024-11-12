use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IMDbConfiguration {
    pub id: String,
    pub category: String,
    pub name: String,
    pub description: String,
    pub casts: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
    pub poster: Poster,
    pub source: Source,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Poster {
    pub source: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    pub url: String
}