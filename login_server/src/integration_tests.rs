pub use serial_test::serial;
use tonic::{
    Request,
    Code
};

use crate::{
    proto::{authenticate_client::AuthenticateClient, LoginRequest}, 
    serve_server,
    SERVER_ADRESS
};

#[tokio::test]
#[serial]
async fn login_fail_ok() {

    let server_thread = tokio::spawn(async move {
        serve_server().await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

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
