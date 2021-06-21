use http::response::Builder;
use http::{HeaderValue, Request, Response};
use hyper::body::aggregate;
use hyper::body::Buf;
use hyper::Body;
use std::mem;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::static_file::FILE_BUFFER_SIZE;
use crate::utils::compression::gzip;

use super::MiddlewareAfter;

/// Creates a CORS middleware with the configuration provided and returns it.
/// The configured headers will be appended to every HTTP Response before
/// sending such response back to the client (After Middleware)
///
/// CORS headers for every response are built on server initialization and
/// then are "appended" to Response headers on every response.
///
/// # Panics
///
/// Panics if a CORS config is not defined for the `Config` instance provided.
/// (`Config.cors` is `None`).
/// `make_cors_middlware` should only be called when a `CorsConfig` is defined.
///
/// Also panics if any CORS header value is not a valid UTF-8 string
pub fn make_gzip_compression_middleware() -> MiddlewareAfter {
    Box::new(
        move |request: Arc<Request<Body>>, response: Arc<Mutex<Response<Body>>>| {
            Box::pin(async move {
                if let Some(accept_encoding_header) =
                    request.headers().get(http::header::ACCEPT_ENCODING)
                {
                    if let Some(_) = accept_encoding_header
                        .to_str()
                        .unwrap()
                        .split(", ")
                        .into_iter()
                        .find(|encoding| *encoding == "gzip")
                    {
                        let mut buffer: Vec<u8> = Vec::new();

                        {
                            let mut response = response.lock().await;
                            let body = response.body_mut();
                            let mut buffer_cursor = aggregate(body).await.unwrap();

                            while buffer_cursor.has_remaining() {
                                buffer.push(buffer_cursor.get_u8());
                            }
                        }

                        let compressed = gzip(&buffer)?;
                        let mut response = response.lock().await;
                        let response_headers = response.headers_mut();

                        response_headers.append(
                            http::header::CONTENT_ENCODING,
                            HeaderValue::from_str("gzip").unwrap(),
                        );

                        response_headers.remove(http::header::CONTENT_LENGTH);

                        println!("Response Size: {}", compressed.len());
                        // response_headers.append(
                        //     http::header::CONTENT_LENGTH,
                        //     HeaderValue::from(compressed.len()),
                        // );

                        *response.body_mut() = Body::from(compressed);
                    }
                }

                Ok(())
            })
        },
    )
}
