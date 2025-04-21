use crate::{
    data_access::{DataAccessManager, UserController}, 
    proto::{
        user_server::User, 
        DeleteYourselfRequest, 
        DeleteYourselfResponse,
        GetYourselfRequest, 
        GetYourselfResponse
    }, 
    request_context::RequestContext, 
    views::user::UserForGet, 
    Error
};
use tonic::{Request, Response, Status};

pub struct UserService {
    pub dam: DataAccessManager
}

#[tonic::async_trait]
impl User for UserService {
    async fn get_yourself(
        &self, 
        req: Request<GetYourselfRequest>
    ) -> Result<Response<GetYourselfResponse>,Status> {
        
        let request_context = req.extensions()
        .get::<RequestContext>()
        .ok_or(Error::MissingRequestContext)?;

        let user: UserForGet = UserController::get(&self.dam, request_context.user_id).await?;
        
        let get_yourself_res = GetYourselfResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email
        };

        Ok(Response::new(get_yourself_res))
    }

    async fn delete_yourself(
        &self, 
        req: Request<DeleteYourselfRequest>
    ) -> Result<Response<DeleteYourselfResponse>, tonic::Status> {

        let request_context = req.extensions()
        .get::<RequestContext>()
        .ok_or(Error::MissingRequestContext)?;

        UserController::delete(&self.dam, request_context.user_id).await?;

        Ok(Response::new(DeleteYourselfResponse {}))
    }
}