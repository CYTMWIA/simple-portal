pub mod config;

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::get,
    routing::put,
    Router,
};
use log::{info, warn};
use std::{env, fs, path::Path};
use tera::{Context, Tera};

#[macro_use]
extern crate lazy_static;

static CONFIG_PATHS: [&str; 4] = [
    "config.yaml",
    "config/config.yaml",
    "config-example.yaml",
    "config/config-example.yaml",
];

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        let bytes = include_bytes!("../templates/index.html");
        let index_html = String::from_utf8_lossy(bytes);
        tera.add_raw_template("index.html", index_html.as_ref())
            .unwrap();
        tera
    };
}

fn hash_api_key(s: &str) -> String {
    let res = blake3::hash(s.as_bytes());
    res.to_string()
}

fn read_config() -> Result<config::Config, String> {
    for p in CONFIG_PATHS {
        let path = Path::new(p);
        if !path.exists() {
            warn!("File '{}' not exists.", p);
            continue;
        }

        let file = match fs::read(path) {
            Ok(res) => res,
            Err(e) => return Err(format!("Could not read '{p}': {}", e.to_string())),
        };

        let yaml: config::Config = match serde_yaml::from_slice(file.as_slice()) {
            Ok(res) => res,
            Err(e) => return Err(format!("Parsing '{p}' failed: {}", e.to_string())),
        };

        return Ok(yaml);
    }
    return Err("Could not find any valid config file.".to_owned());
}

async fn index() -> Html<std::string::String> {
    let config = match read_config() {
        Ok(res) => res,
        Err(e) => return Html(e),
    };

    let context = Context::from_serialize(config).unwrap();
    let render = TEMPLATES.render("index.html", &context);

    Html(match render {
        Ok(res) => res,
        Err(err) => err.to_string(),
    })
}

async fn upload_config(headers: HeaderMap, body: Bytes) -> StatusCode {
    let pass: bool;
    match read_config() {
        Ok(res) => {
            let auth_header = match headers.get("X-Authorization") {
                Some(res) => res,
                None => return StatusCode::FORBIDDEN,
            };

            match auth_header.to_str() {
                Ok(auth) => pass = res.app.keys.contains(&hash_api_key(auth)),
                Err(_) => return StatusCode::FORBIDDEN,
            }
        }
        Err(_) => pass = true,
    }
    if !pass {
        return StatusCode::FORBIDDEN;
    }

    match fs::write(CONFIG_PATHS[0], body) {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    info!("CWD: {}", env::current_dir().unwrap().display());

    let app = Router::new()
        .route("/", get(index))
        .route("/config", put(upload_config));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
