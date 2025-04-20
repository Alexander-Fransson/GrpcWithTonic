use tonic::Request;
use crate::{
    request_context::RequestContext, 
    Error, Result
};

pub fn check_request_context_interceptor(req: Request<()>) -> Result<Request<()>> {
    
    req.extensions()
    .get::<RequestContext>()
    .ok_or(Error::MissingRequestContext)?;
    
    Ok(req)
}