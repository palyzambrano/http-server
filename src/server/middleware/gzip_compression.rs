use http::Response;
use hyper::Body;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    Box::new(move |_: _, response: Arc<Mutex<Response<Body>>>| {
        Box::pin(async move {
            let mut response = response.lock().await;

            *response.body_mut() = Body::empty();

            Ok(())
        })
    })
}
