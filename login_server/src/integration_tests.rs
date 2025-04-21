pub use serial_test::serial;
use tonic::{
    Request,
    Code
};

use crate::{
    grpc::JWT_METADATA_KEY, proto::{
        authenticate_client::AuthenticateClient, 
        user_client::UserClient, 
        DeleteYourselfRequest, 
        LoginRequest, 
        RegisterRequest
    }, serve_server, SERVER_ADRESS
};

#[tokio::test]
#[serial]
async fn register_delete_user_ok() {

    let server_thread = tokio::spawn(async move {
        serve_server().await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let mut delete_client = UserClient::connect(
        format!("http://{}", SERVER_ADRESS)
    ).await.unwrap();

    let failed_delete_req = Request::new(DeleteYourselfRequest{});
    let failed_delete_res = delete_client.delete_yourself(failed_delete_req).await;

    match failed_delete_res {
        Err(status) => assert_eq!(status.code(), Code::PermissionDenied),
        Ok(_) => assert!(false)
    }

    let mut auth_client = AuthenticateClient::connect(
        format!("http://{}", SERVER_ADRESS)
    ).await.unwrap();

    let register_req = Request::new(RegisterRequest{
        email: "email@example.com".to_string(),
        password: "1234".to_string(),
        name: "John Doe".to_string(),
    });
    
    let register_res = auth_client.register(register_req).await.unwrap();

    let token = register_res.into_inner().token;

    let mut success_delete_req = Request::new(DeleteYourselfRequest{});
    success_delete_req.metadata_mut().insert(JWT_METADATA_KEY, token.parse().unwrap());

    let success_delete_res = delete_client.delete_yourself(success_delete_req).await;

    match success_delete_res {
        Err(_) => assert!(false),
        Ok(_) => assert!(true)
    }

    server_thread.abort();
}

#[tokio::test]
#[serial]
async fn login_fail_ok() {

    let server_thread = tokio::spawn(async move {
        serve_server().await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let mut client = AuthenticateClient::connect(
        format!("http://{}", SERVER_ADRESS)
    ).await.unwrap();

    let null_user_req = Request::new(LoginRequest{
        email: "a@b_that_does_not_exist.com".to_string(),
        password: "1234".to_string(),
    });

    let null_user_res = client.login(null_user_req).await;

    match null_user_res {
        Err(status) => assert_eq!(status.code(), Code::NotFound),
        Ok(_) => panic!("Expected NotFound error, but got Ok response"),
    }

    let bad_password_req = Request::new(LoginRequest{
        email: "email@example.com".to_string(), // seeded in login_server/db/sql/migrations/0002_seed_user.sql
        password: "bad_password".to_string(),
    });

    let bad_password_res = client.login(bad_password_req).await;

    match bad_password_res {
        Err(status) => assert_eq!(status.code(), Code::PermissionDenied),
        Ok(_) => panic!("Expected PermissionDenied error, but got Ok response"),
    }

    server_thread.abort();
}
