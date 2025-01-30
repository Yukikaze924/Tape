use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IMDbConfiguration
{
    pub id: String,
    #[serde(rename = "type")]
    pub _type: usize,
    pub title: String,
    pub category: String,
    pub rating: f32,
    pub classification: String,
    pub interests: Vec<String>,
    pub description: String,
    pub casts: Vec<String>,
    pub directors: Vec<String>,
    pub writers: Vec<String>,
    pub poster: Poster,
    pub source: Source,
    pub stream: Stream,
    pub tracks: Vec<Track>
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Stream {
    pub mp4: Source
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub kind: String,
    pub src: String,
    #[serde(rename = "srcLang")]
    pub src_lang: String,
    pub label: String
}