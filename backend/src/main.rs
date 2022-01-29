use axum::{
    extract,
    extract::{ContentLengthLimit, Multipart},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod huffman;
pub use crate::huffman::{
    decoding::decompress,
    encoding::compress,
    models::{AppError, CompressRequest, Encoded},
};

const UPLOAD_LIMIT: u64 = 10 * 1_000_000; // 10MB; 1MB = 1_000_000 Bytes
static FORM_FIELD_NAME: &str = "txt_file";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/compress", post(compress_text))
        .route("/compress_file", post(compress_file))
        // This handler is used to carry out decompression. The orignal text is churned out as
        // an array of utf-8 bytes.
        .route(
            "/decompress",
            post(|extract::Json(payload): extract::Json<Encoded>| async {
                (StatusCode::OK, Json(decompress(payload))).into_response()
            }),
        );

    // create and run the server
    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        8000,
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
}

/// This is the handler for the /compress endpoint.
/// It handles the text input parsing for compression.
async fn compress_text(
    extract::Json(payload): extract::Json<CompressRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Send the text data for encoding.
    // And return the encoded data.
    Ok((
        StatusCode::OK,
        Json(compress(payload.text.as_bytes(), None)),
    ))
}

/// This is the handler for the /compress_file endpoint.
/// It handles the file input parsing for compression.
async fn compress_file(
    ContentLengthLimit(mut payload): ContentLengthLimit<Multipart, UPLOAD_LIMIT>,
) -> Result<impl IntoResponse, AppError> {
    // Extract the form field with data.
    let field = payload.next_field().await.unwrap().unwrap();

    if FORM_FIELD_NAME.eq(field.name().unwrap()) {
        // Fetch the filename first.
        let filename = match field.file_name() {
            Some(filename) => Some(filename.to_string()),
            None => None,
        };

        // Send the file contents for encoding.
        let data = compress(&field.bytes().await.unwrap(), filename);

        // Return the encoded data as response.
        Ok((StatusCode::OK, Json(data)).into_response())
    } else {
        Err(AppError::InternalServerError(format!(
            "File expected is of form field name, '{}'",
            FORM_FIELD_NAME
        )))
    }
}
