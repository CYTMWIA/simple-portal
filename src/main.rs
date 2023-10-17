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

fn read_config() -> Result<config::Config, io::Error> {
    for p in CONFIG_PATHS {
        let path = Path::new(p);
        if !path.exists() {
            continue;
        }

        let reading = fs::read(path);
        if reading.is_err() {
            continue;
        }

        let parsing: Result<config::Config, serde_yaml::Error> =
            serde_yaml::from_slice(reading.unwrap().as_slice());
        if parsing.is_err() {
            continue;
        }

        return Ok(parsing.unwrap());
    }
    return Err(io::Error::new(
        io::ErrorKind::NotFound,
        "config file not found.",
    ));
}

async fn index() -> Html<std::string::String> {
    let config = read_config().unwrap();

    let context = Context::from_serialize(config).unwrap();
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
