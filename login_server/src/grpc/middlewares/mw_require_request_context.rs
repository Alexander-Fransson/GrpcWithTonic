use tonic::{Request, Status};
use crate::{
    request_context::RequestContext, 
    Error
};

pub fn check_request_context_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    
    req.extensions()
    .get::<RequestContext>()
    .ok_or(Error::MissingRequestContext)?;
    
    Ok(req)
}