use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub app: App,
    pub groups: Vec<UrlGroup>,
}
#[derive(Serialize, Deserialize)]
pub struct App {
    pub title: String,
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UrlGroup {
    pub title: String,
    pub items: Vec<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub title: String,
    pub url: String,
}
