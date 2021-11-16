#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod huffman;

pub use crate::huffman::coding::compress_text;
pub use crate::huffman::decoding::decompress_text;
pub use crate::huffman::models::CompressRequest;

use huffman::models::{Decoded, Encoded};
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket::post;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[post("/compress", format = "json", data = "<request>")]
fn encode_text<'a>(request: Json<CompressRequest<'a>>) -> Json<Encoded> {
    Json(compress_text(request.0.text))
}

#[post("/decompress", format = "json", data = "<request>")]
fn decode_text(request: Json<Encoded>) -> Json<Decoded> {
    Json(decompress_text(request.0))
}

// TODO: work on file uploads

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
