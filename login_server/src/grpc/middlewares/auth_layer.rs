// after youve made the tower middleware tutorial the you may apply it to the login server

use tower::Service;

#[derive(Clone)]
pub struct AuthLayer;

impl<S> tower::Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;
    fn layer(&self, inner_service: S) -> Self::Service {
        AuthMiddleware { inner_service }
    }
}

pub struct AuthMiddleware<S> {
    inner_service: S // what the middleware is wrapping
}

