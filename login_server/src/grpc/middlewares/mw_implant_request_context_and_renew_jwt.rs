use std::str::FromStr;
use http::{HeaderValue, Request, Response };
use tonic::body::Body;
use tonic::async_trait;
use tonic_middleware::ServiceBound;
use crate::crypt::jwt::JwtToken;
use crate::views::user::UserForAuth;
use crate::data_access::{DataAccessManager, UserController};
use crate::request_context::RequestContext;
use super::super::JWT_METADATA_KEY;
use super::generate_generic_http_error_for_grpc;
use tracing::debug;


use tonic_middleware::Middleware;

#[derive(Clone)]
pub struct MiddlewareImplantingRequestContextAndRenewingJwt {
    pub dam: DataAccessManager
}

#[async_trait]
impl<S> Middleware<S> for MiddlewareImplantingRequestContextAndRenewingJwt 
where 
    S: ServiceBound,
    S::Future: Send
{
    async fn call(
        &self,
        mut req: Request<Body>,
        mut service: S
    ) -> Result<Response<Body>, S::Error> {

        let header_values = req.headers()
        .get(JWT_METADATA_KEY)
        .map(|v| v.to_str());

        let jwt_str = if let Some(Ok(jwt_str)) = header_values {

            let validated_jwt = validate_and_renew_jwt(&self.dam, jwt_str).await;

            if let Ok(new_jwt) = validated_jwt {

                req.extensions_mut().insert(RequestContext::new(new_jwt.user_id));
                let jwt_str = new_jwt.to_string();

                Some(jwt_str)

            } else {
                debug!("VALIDATION ERROR IN REQUEST CONTEXT MIDDLEWARE: {:?}", validated_jwt);
                return Ok(generate_generic_http_error_for_grpc())
            }

        } else {None};

        let mut res = service.call(req).await?;

        if let Some(jwt_str) = jwt_str {
            if let Ok(jwt_header_value) = HeaderValue::from_str(&jwt_str) {
                res.headers_mut().insert(JWT_METADATA_KEY, jwt_header_value);
            }
        }

        Ok(res)
    }
}

async fn validate_and_renew_jwt(
    dam: &DataAccessManager,
    jwt_str: &str
) -> crate::Result<JwtToken> {
    let jwt = JwtToken::from_str(jwt_str)?;
    let user_for_auth: UserForAuth = UserController::get(dam, jwt.user_id).await?;
    
    jwt.validate_and_reset_durration(
        &user_for_auth.encryption_salt.to_string()
    )
}
