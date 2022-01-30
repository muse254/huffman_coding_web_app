use axum::{
    body::Body, extract, http::StatusCode, response::IntoResponse, routing::post, Json, Router,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
mod huffman;
pub use crate::huffman::{
    decoding::decompress,
    encoding::compress,
    models::{AppError, CompressRequest, Encoded},
};
use http::{header, HeaderValue};
use tower::ServiceBuilder;
use tower_http::{
    cors::{any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods(any())
        .allow_origin(any())
        .allow_credentials(true)
        .allow_headers(any());

    let cors_middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::if_not_present(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        ))
        .layer(cors)
        .into_inner();

    let app = Router::new()
        // This is the handler for the /compress endpoint.
        // It handles the text input parsing for compression.
        .route(
            "/compress",
            post(
                |extract::Json(payload): extract::Json<CompressRequest>| async {
                    // Send the text data for encoding.
                    // And return the encoded data.
                    (StatusCode::OK, Json(compress(Vec::from(payload.text)))).into_response()
                },
            ),
        )
        // This handler is used to carry out decompression. The orignal text is churned out as
        // an array of utf-8 bytes.
        .route(
            "/decompress",
            post(|extract::Json(payload): extract::Json<Encoded>| async {
                (StatusCode::OK, Json(decompress(payload))).into_response()
            }),
        )
        .layer(cors_middleware);

    // create and run the server
    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        8000,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
