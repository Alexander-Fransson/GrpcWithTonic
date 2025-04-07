use crate::data_access::_get_data_access_manager_instance_for_tests;
use crate::Result;
use crate::views::user::UserForCreate;
use super::*;
use serial_test::serial;

// some "role postgres user does not exist error"

#[tokio::test]
#[serial] // tests will fail if their not serial
async fn create_get_delete_user_ok() -> Result<()> {

    let user = UserForCreate {
        name: "name".to_string(),
        email: "7R5Y8@example.com".to_string(),
        password: "password".to_string(),
    };

    let dam = _get_data_access_manager_instance_for_tests().await;
    let create_req_id = UserController::create(&dam, user.clone()).await?;
    let get_req_user = UserController::get(&dam, create_req_id).await?;

    assert_eq!(user.name, get_req_user.name);

    UserController::delete(&dam, create_req_id).await?;

    Ok(())
}