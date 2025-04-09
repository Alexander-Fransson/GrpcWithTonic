use crate::{
    data_access::{
        DataAccessManager,
        UserController
    }, 
    proto::{
        authenticate_server::{
            Authenticate,
            AuthenticateServer
        }, 
        LoginRequest, 
        LoginResponse, 
        RegisterRequest, 
        RegisterResponse
    },
    views::user::UserForLogin
};
use tonic::{Request, Response, Status};

pub struct AuthService {
    dam: DataAccessManager
}

#[tonic::async_trait]
impl Authenticate for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        
        let inner_request = request.into_inner();

        let user_for_register = UserForLogin {
            email: inner_request.email,
            password: inner_request.password
        };

        let login_res = UserController::login(&self.dam, user_for_register).await?;

        // TODO: return token and implement into status?
               
        Ok(Response::new(()))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        Ok(Response::new(()))
    }

}
