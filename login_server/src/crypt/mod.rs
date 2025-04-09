#[cfg(test)]
mod tests;
pub mod password;
pub mod jwt;

use argon2::{
    password_hash::SaltString, 
    Argon2,
    PasswordHasher
};
use blake2::{
    digest::Mac, 
    Blake2bMac512
};
use crate::utils::base64::u8_to_b64;
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

fn encrypt_blake_2b_mac_512(key: &[u8], content: &EncryptionContent) -> Result<String> {

    let EncryptionContent {content, salt} = content;

    let mut hasher = Blake2bMac512::new_from_slice(key)
    .map_err(|e| Error::FailedToCreateMacKey(e.to_string()))?;

    hasher.update(content.as_bytes());
    hasher.update(salt.as_bytes());

    let result_bytes = hasher.finalize().into_bytes();
    let b64_result = u8_to_b64(&result_bytes);

    Ok(b64_result)
}