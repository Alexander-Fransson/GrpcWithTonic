use crate::data_access::_get_data_access_manager_instance_for_tests;
use crate::Result;
use crate::views::user::{FullUser, UserForCreate, UserForGet};
use super::*;
use serial_test::serial;

const TEST_EMAIL: &str = "aadawfawfaknwalkjgnwangagjnkawnjgkajfnkajds,mnfaf_unique_email@example.com";
const TEST_PASSWORD: &str = "test_password";

fn generate_user_for_register() -> UserForRegister {
    UserForRegister {
        name: "test_user".to_string(),
        email: TEST_EMAIL.to_string(),
        password: TEST_PASSWORD.to_string(),
    }
}

#[serial]
#[tokio::test]
async fn register_user_ok() -> Result<()> {
    // for some reason theese functions take up so much time that the pools fail if they are run with the common pool
    // maybe it is a feature of the password hashing.
    let dam = DataAccessManager::new().await?; 

    let new_user = generate_user_for_register();
    
    let create_req = UserController::register(&dam, new_user).await?;

    let user: FullUser = UserController::get(&dam, create_req.id).await?;

    assert_eq!(user.name, "test_user");
    assert_eq!(user.email, TEST_EMAIL);
    assert!(user.password.starts_with("#0#$argon2id")); // is encrypted with argon2
    assert_eq!(user.encryption_salt.get_version_num(), 4); // uuid of type 4

    UserController::delete(&dam, user.id).await?;

    Ok(())
}

#[serial]
#[tokio::test]
async fn login_user_ok() -> Result<()> {
    let dam = DataAccessManager::new().await?;

    let new_user = generate_user_for_register();
    let create_req = UserController::register(&dam, new_user).await?;

    let login_credentials = UserForLogin {
        email: TEST_EMAIL.to_string(),
        password: TEST_PASSWORD.to_string(),
    };

    let login_res = UserController::login(&dam, login_credentials.clone()).await?;

    assert_eq!(login_res.id, create_req.id);

    let bad_password_credentials = UserForLogin {
        email: TEST_EMAIL.to_string(),
        password: "bad_password".to_string(),
    };

    let bad_login_res = UserController::login(
        &dam, 
        bad_password_credentials
    ).await;

    assert!(matches!(bad_login_res, Err(Error::PasswordInvalid)));

    UserController::delete(&dam, create_req.id).await?;

    let not_found_login_res = UserController::login(&dam, login_credentials).await;
    assert!(matches!(not_found_login_res, Err(Error::EntityNotFound)));

    Ok(())
}


#[tokio::test]
#[serial] // tests will fail if their not serial
async fn create_get_delete_user_ok() -> Result<()> {

    let user = generate_user_for_register();
    
    let user = UserForCreate{
        name: user.name,
        email: user.email,
        password: user.password,
        encryption_salt: Uuid::new_v4()
    };

    let dam = _get_data_access_manager_instance_for_tests().await;
    let create_req: UserForGet = UserController::create(&dam, user.clone()).await?;
    let get_req_user: UserForGet = UserController::get(&dam, create_req.id).await?;

    assert_eq!(user.name, get_req_user.name);

    UserController::delete(&dam, create_req.id).await?;

    Ok(())
}