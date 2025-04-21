pub mod mw_implant_request_context_and_renew_jwt;
pub mod mw_require_request_context;

use tonic::body::Body;
use http::{HeaderValue, Response, StatusCode};

pub use mw_implant_request_context_and_renew_jwt::MiddlewareImplantingRequestContextAndRenewingJwt;
pub use mw_require_request_context::check_request_context_interceptor;

pub fn generate_generic_http_error_for_grpc() -> Response<Body> {
    
    let mut res = Response::new(Body::empty());
    
    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_static("application/grpc"),
    );
    res.headers_mut().insert(
        "grpc-status",
        HeaderValue::from_static("13"), // 13 is 500 for grpc
    );
    res.headers_mut().insert(
        "grpc-message",
        HeaderValue::from_static("Internal Server Error"),
    );

    res
}