use std::str::FromStr;
use http::{Request, Response};
use tonic::async_trait;
use tonic::body::Body;
use tonic_middleware::{Middleware, ServiceBound};
use crate::crypt::jwt::JwtToken;
use crate::views::user::UserForAuth;
use crate::data_access::{DataAccessManager, UserController};
use crate::request_context::RequestContext;
use super::super::JWT_METADATA_KEY;

#[derive(Default, Clone)]
pub struct MetricsMiddleware;



#[derive(Clone)]
pub struct MwImplantRequestContext{
    dam: DataAccessManager
}

#[async_trait]
impl<S> Middleware<S> for MwImplantRequestContext
where 
    S: ServiceBound,
    S::Future: Send,
{
    async fn call(
        &self, 
        mut req: Request<Body>, 
        mut service: S,
    ) -> Result<Response<Body>, S::Error> {

        // make the result better

        let auth_header = req.headers()
        .get(JWT_METADATA_KEY);

        if let Some(header_value) = auth_header {
            let token_str = header_value.to_str().unwrap_or_default();
            let rc = request_context_from_jwt(&self.dam, token_str).await;

            if let Ok(rc) = rc {
                req.extensions_mut().insert(rc);
            }
        }

        let result = service.call(req).await?;
        
        Ok(result)
    }
}

async fn request_context_from_jwt(dam: &DataAccessManager, jwt_str: &str) -> crate::Result<RequestContext> {
    let jwt = JwtToken::from_str(jwt_str)?;
    let user_for_auth: UserForAuth = UserController::get(dam, jwt.user_id).await?;
    
    jwt.validate(&user_for_auth.encryption_salt.to_string())?;

    Ok(RequestContext::new(jwt.user_id))
}