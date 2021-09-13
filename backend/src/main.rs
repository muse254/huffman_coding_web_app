#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use backend::{
    huffman_coding::generate_codes,
    models::{CompressRequest, HuffmanCodes},
};
use rocket::config::{Config, Environment};
use rocket::http::Method;
use rocket::post;
use rocket_contrib::json::Json;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[post("/compress", format = "application/json", data = "<request>")]
fn get_compression<'a>(request: Json<CompressRequest<'a>>) -> Json<HuffmanCodes> {
    let text = request.0.text;
    let huffman_codes = generate_codes(text);

    Json(huffman_codes)
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
        .mount("/", routes![get_compression])
        .launch();
}
