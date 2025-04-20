use uuid::Uuid;
use crate::{
    crypt::{
        jwt::{
            create_jwt_signature, 
            create_jwt_token, 
            hash_for_jwt, 
            validate_jwt_by_signature_and_expiration_and_reset_durration
        }, password::{
            hash_password, 
            validate_password
        }, EncryptionContent
    }, utils::time::now_utc_plus_sec_str, Error, Result
};

#[test]
fn hash_password_ok() -> Result<()> {

    let encryption_content = EncryptionContent {
        content: "somecontent".to_string(),
        salt: "aGVsbG8gd29ybGR+Cg==".to_string()
    };
    
    let hashed_password = hash_password(&encryption_content)?;
    
    assert_eq!(&hashed_password,"#0#$argon2id$v=19$m=19456,t=2,p=1$aGVsbG8gd29ybGR+Cg$KNQ4cSSFwLmzqDgtJ6SnIJe6ElCTk3peC2ui4LyI0OA");
    
    Ok(())
}

#[test]
fn validate_password_ok() -> Result<()> {
    
    let password_ref = "#0#$argon2id$v=19$m=19456,t=2,p=1$aGVsbG8gd29ybGR+Cg$KNQ4cSSFwLmzqDgtJ6SnIJe6ElCTk3peC2ui4LyI0OA".to_string();

    let enc_content = EncryptionContent {
        content: "somecontent".to_string(),
        salt: "aGVsbG8gd29ybGR+Cg==".to_string()
    };
    
    validate_password(password_ref, &enc_content)?;

    let failed_password = validate_password(
        "wrongpassword".to_string(), 
        &enc_content
    );

    if let Err(Error::PasswordInvalid) = failed_password {assert!(true);} 
    else {assert!(false);}

    Ok(()) 
}

#[test]
    fn encrypt_blake2b_mac_512_ok() -> Result<()> {
        
        let test_key = [0u8; 64];

        let test_enc_content = EncryptionContent {
            content: "somecontent".to_string(),
            salt: "electrolytes".to_string()
        };

        let signature = hash_for_jwt(&test_key, &test_enc_content)?;

        assert_eq!(&signature, "#1#3InVh31+qBSdgCzxOD6bUigENgiJcza+BfA6Uj2ETWZ6geu0ID5vOIG/CGiB2gg5eBMeh9Map4GeuBasuQbpsQ==");
        
        Ok(())
    }

#[test]
fn test_create_sign_and_validate_jwt_token() -> Result<()> {
    let user_id = Uuid::new_v4();
    let salt = "salt";
    let jwt_key = [0u8; 64];
    let durration_sec = 2.0;

    let token = create_jwt_token(user_id, salt, &jwt_key, durration_sec)?;

    let reference_signature = create_jwt_signature(&user_id, salt, &jwt_key)?;

    assert_eq!(&token.signature, &reference_signature);
    assert_eq!(token.user_id, user_id);
    assert!(token.expiration <= now_utc_plus_sec_str(durration_sec)?);
    assert_eq!(token.expiration, token.expiration);

    validate_jwt_by_signature_and_expiration_and_reset_durration(
        &token, 
        salt, 
        &jwt_key, 
        durration_sec
    )?;

    Ok(())
}