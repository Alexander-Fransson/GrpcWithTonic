#[cfg(test)]
mod tests;
mod password;

use argon2::{
    password_hash::SaltString, 
    Argon2,
    PasswordHasher
};

use crate::{Error, Result};

#[derive(Debug)]
pub struct EncryptionContent {
    pub content: String,
    pub salt: String
}

fn hash_password_argon2(content: &EncryptionContent) -> Result<String> {
    
    let EncryptionContent {content, salt} = content;
    
    // argon2 does not accept b64 padding
    let trimmed_salt = salt.trim_end_matches("=");
    let salt_string = SaltString::from_b64(trimmed_salt)
    .map_err(|e| Error::FailedToCreateSaltString(e.to_string()))?;

    let argon2 = Argon2::default();

    let hash = argon2.hash_password(
        content.as_bytes(), 
        &salt_string
    ).map_err(|e| Error::FailedToHashPassword(e.to_string()))?;

    Ok(hash.to_string())
}

