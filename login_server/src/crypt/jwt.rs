use std::str::FromStr;
use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::{
    get_env::get_env_static, utils::{base64::{
        b64_to_string, 
        str_to_base_64
    }, time::{now_utc_plus_sec_str, time_str_to_offset_date_time}}, Error, Result
};
use super::{
    EncryptionContent,
    encrypt_blake_2b_mac_512
};

pub struct JwtToken {
    pub user_id: Uuid,
    pub expiration: String,
    pub signature: String
}

impl JwtToken {
    pub fn new(user_id: Uuid, salt: &str) -> Result<Self> {
        let jwt_key = &get_env_static().JWT_KEY;
        let durration_sec = get_env_static().JWT_DURRATION_SEC;

        create_jwt_token(user_id, salt, jwt_key, durration_sec)
    }

    pub fn validate(&self, salt: &str) -> Result<()> {
        let jwt_key = &get_env_static().JWT_KEY;

        validate_jwt_token_by_signature_and_expiration(self, salt, jwt_key)
    }
}

impl FromStr for JwtToken {
    type Err = Error;
    fn from_str(token_str: &str) -> Result<Self> {
        let token_parts = token_str.split(".").collect::<Vec<&str>>();

        if token_parts.len() != 3 {
            return Err(Error::JwtTokenWrongFormat);
        }

        let (
            b64_user_id,
            b64_expiration,
            signature_str
        ) = (token_parts[0], token_parts[1], token_parts[2]);

        let user_id = Uuid::from_str(b64_user_id)
        .map_err(|e| Error::FailedToParse(e.to_string()))?;

        let expiration = b64_to_string(b64_expiration)?;
        let signature = signature_str.to_string();

        Ok(Self{
            user_id,
            expiration,
            signature
        })
    }
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}.{}.{}", 
            str_to_base_64(&self.user_id.to_string()), 
            str_to_base_64(&self.expiration), 
            self.signature
        )
    }
}

pub fn hash_for_jwt(key: &[u8], content: &EncryptionContent) -> Result<String> {
    let hash = encrypt_blake_2b_mac_512(key, content)?;
    Ok(format!("#1#{}", hash))
}

pub(crate) fn create_blake2b_signature(
    user_id: &Uuid,
    salt: &str,
    jwt_key: &[u8]    
) -> Result<String> {

    let content = user_id.to_string();

    let enc_content = EncryptionContent {
        content,
        salt: salt.to_string()
    };

    encrypt_blake_2b_mac_512(jwt_key, &enc_content)
}

pub(crate) fn create_jwt_token(user_id: Uuid, salt: &str, jwt_key: &[u8], durration_sec: f64) -> Result<JwtToken> {
    let signature = create_blake2b_signature(&user_id, salt, jwt_key)?;
    let expiration = now_utc_plus_sec_str(durration_sec)?;

    Ok(JwtToken {
        user_id,
        expiration,
        signature
    })
}

pub (crate) fn validate_jwt_token_by_signature_and_expiration(
    token: &JwtToken, 
    salt: &str, 
    jwt_key: &[u8]
) -> Result<()> {

    let reference_signature = create_blake2b_signature(
        &token.user_id, 
        salt, 
        jwt_key
    )?;

    if token.signature != reference_signature {
        return Err(Error::InvalidJwtTokenSignature);
    }

    let expiration = time_str_to_offset_date_time(&token.expiration)?;
    
    if expiration < OffsetDateTime::now_utc() {
        return Err(Error::JwtTokenExpired);
    }

    Ok(())
}