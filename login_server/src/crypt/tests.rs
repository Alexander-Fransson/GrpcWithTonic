use crate::crypt::EncryptionContent;
use crate::crypt::password::{hash_password, validate_password};
use crate::{Result, Error};

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