pub mod config;

use axum::{response::Html, routing::get, Router};
use std::{env, fs, io, path::Path};
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

fn read_config() -> Result<Vec<u8>, io::Error> {
    for p in CONFIG_PATHS {
        let path = Path::new(p);
        if path.exists() {
            match fs::read(path) {
                Ok(res) => return Ok(res),
                Err(_) => {}
            }
        }
    }
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "config file not found.",
    ));
}

async fn index() -> Html<std::string::String> {
    let content = read_config().unwrap();

    let data: config::Config = serde_yaml::from_slice(content.as_slice()).unwrap();
    let context = Context::from_serialize(data).unwrap();
    let render = TEMPLATES.render("index.html", &context);

    Html(match render {
        Ok(res) => res,
        Err(err) => err.to_string(),
    })
}

#[tokio::main]
async fn main() {
    println!("CWD: {}", env::current_dir().unwrap().display());

    // build our application with a single route
    let app = Router::new().route("/", get(index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
