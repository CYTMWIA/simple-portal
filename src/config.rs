use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Content {
    app: App,
    groups: Vec<UrlGroup>,
}
#[derive(Serialize, Deserialize)]
pub struct App {
    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct UrlGroup {
    name: String,
    items: Vec<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct Url {
    title: String,
    url: String,
}
