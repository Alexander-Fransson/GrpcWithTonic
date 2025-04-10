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

    //  this will probably not work twice

    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    let mut client = AuthenticateClient::connect(
        format!("http://{}", SERVER_ADRESS)
    ).await.unwrap();

    let req = Request::new(LoginRequest{
        email: "a@b_that_does_not_exist.com".to_string(),
        password: "1234".to_string(),
    });

    let res = client.login(req).await;

    // assert that it fails because the user does not exist

    match res {
        Err(status) => assert_eq!(status.code(), Code::NotFound),
        Ok(_) => panic!("Expected NotFound error, but got Ok response"),
    }

    server_thread.abort();
}
