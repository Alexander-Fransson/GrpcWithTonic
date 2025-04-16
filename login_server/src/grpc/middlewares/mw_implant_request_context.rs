use std::str::FromStr;
use http::Request;
use tonic::{async_trait, Status};
use tonic::body::Body;
use tonic_middleware::RequestInterceptor;
use crate::crypt::jwt::JwtToken;
use crate::views::user::UserForAuth;
use crate::data_access::{DataAccessManager, UserController};
use crate::request_context::RequestContext;
use super::super::JWT_METADATA_KEY;

// ensure token is sliding
// also test 

#[derive(Clone)]
pub struct InterceptorImplantingRequestContext {
    pub dam: DataAccessManager
}

#[async_trait]
impl RequestInterceptor for InterceptorImplantingRequestContext {
    async fn intercept(&self, mut req: Request<Body>) -> Result<Request<Body>, Status> {
        
        let header_values = req.headers()
        .get(JWT_METADATA_KEY)
        .map(|v| v.to_str());

        if let Some(Ok(jwt_str)) = header_values {

            let rc = request_context_from_jwt(&self.dam, jwt_str).await?;
            req.extensions_mut().insert(rc);

        }

        Ok(req)
    }
}

async fn request_context_from_jwt(dam: &DataAccessManager, jwt_str: &str) -> crate::Result<RequestContext> {
    let jwt = JwtToken::from_str(jwt_str)?;
    let user_for_auth: UserForAuth = UserController::get(dam, jwt.user_id).await?;
    
    jwt.validate(&user_for_auth.encryption_salt.to_string())?;

    Ok(RequestContext::new(jwt.user_id))
}