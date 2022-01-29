#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod huffman;

pub use crate::huffman::decoding::decompress;
pub use crate::huffman::encoding::compress;
pub use crate::huffman::models::CompressRequest;

use huffman::models::{Decoded, Encoded, TextFile};
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket::post;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[post("/compress", format = "json", data = "<request>")]
fn encode_text<'a>(request: Json<CompressRequest<'a>>) -> Json<Encoded> {
    let text_file = TextFile {
        file: Vec::from(request.0.text),
        alpha: "".to_string(),
    };

    Json(compress(text_file))
}

#[post("/compress_file", data = "<upload>")]
fn encode_file<'a>(upload: TextFile) -> Json<Encoded> {
    Json(compress(upload))
}

#[post("/decompress", format = "json", data = "<request>")]
fn decode_text(request: Json<Encoded>) -> Json<Decoded> {
    Json(decompress(request.0))
}

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let port = 8000;
    let cfg = Config::build(Environment::Development)
        .address("127.0.0.1")
        .port(port)
        .unwrap();

    rocket::custom(cfg)
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![encode_text, decode_text])
        .launch();
}
