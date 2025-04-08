use std::str::FromStr;
use std::fmt::Display;
use uuid::Uuid;
use crate::{utils::base64::{b64_to_string, str_to_base_64}, Error, Result};

pub struct JwtToken {
    pub user_id: Uuid,
    pub expiration: String,
    pub signature: String
}

impl JwtToken {
    pub fn new(user_id: Uuid, salt: &str) -> Result<Self> {
        todo!()
    }

    pub fn validate(&self, salt: &str) -> Result<()> {
        todo!()
    }
}

impl FromStr for JwtToken {
    type Err = Error;
    fn from_str(token_str: &str) -> Result<Self> {
        let token_parts = token_str.split(".").collect::<Vec<&str>>();

        if token_parts.len() != 3 {
            return Err(Error::TokenWrongFormat);
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