use crate::{Error, Result};
use super::{hash_password_argon2, EncryptionContent};

pub fn hash_password(content: &EncryptionContent) -> Result<String> {
    let hashed_pwd = hash_password_argon2(content)?;
    Ok(format!("#0#{}", hashed_pwd))
}

pub fn validate_password(password_ref:String, enc_content: &EncryptionContent) -> Result<()> {
    let password = hash_password(enc_content)?;
    if password == password_ref {Ok(())} 
    else {Err(Error::PasswordInvalid)}
}