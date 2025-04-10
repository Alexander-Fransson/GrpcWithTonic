use crate::{
    crypt::jwt::JwtToken, data_access::{
        DataAccessManager,
        UserController
    }, proto::{
        authenticate_server::Authenticate, 
        LoginRequest, 
        LoginResponse, 
        RegisterRequest, 
        RegisterResponse
    }, views::user::{UserForLogin, UserForRegister}
};
use tonic::{Request, Response, Status};

pub struct AuthService {
    pub dam: DataAccessManager
}

#[tonic::async_trait]
impl Authenticate for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {

        // remember to validate in middlewars for the other services

        let inner_request = request.into_inner();

        let user_for_login = UserForLogin {
            email: inner_request.email,
            password: inner_request.password
        };

        let login_res = UserController::login(&self.dam, user_for_login).await?;
        let token = JwtToken::new(
            login_res.id, 
            &login_res.encryption_salt.to_string()
        )?.to_string();       
        let login_res = LoginResponse {token};

        Ok(Response::new(login_res))
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        let inner_request = request.into_inner();

        let user_for_register = UserForRegister {
            name: inner_request.name,
            email: inner_request.email,
            password: inner_request.password
        };
        
        let register_res = UserController::register(&self.dam, user_for_register).await?;
        let token = JwtToken::new(
            register_res.id, 
            &register_res.encryption_salt.to_string()
        )?.to_string();
        
        let register_res = RegisterResponse {token};

        Ok(Response::new(register_res))
    }

}
