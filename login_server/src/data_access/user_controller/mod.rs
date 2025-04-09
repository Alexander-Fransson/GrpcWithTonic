use uuid::Uuid;
use crate::{crypt::{
    password::{
        hash_password,
        validate_password
    }, EncryptionContent
}, views::user::UserForRegister};
use crate::utils::base64::str_to_base_64;
use crate::utils::traits::FieldsAsStrings;
use crate::{Result, Error};
use crate::views::user::{
    GettableUser, 
    UserForAuth, 
    UserForCreate, 
    UserForLogin, 
    UserForValidate
};
use super::{
    base_crud::{self, Controller}, 
    DataAccessManager
};

#[cfg(test)]
mod tests;

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\""; // quotation marks are important for sql
}

impl UserController {

    pub async fn register(dam: &DataAccessManager, user: UserForRegister) -> Result<UserForAuth> {
        
        let pwd_salt = Uuid::new_v4();
        let b64_pwd_salt = str_to_base_64(&pwd_salt.to_string());
        let enc_content = EncryptionContent {
            content: user.password,
            salt: b64_pwd_salt
        };
        let enc_password = hash_password(&enc_content)?;
        
        let user_for_create = UserForCreate {
            name: user.name,
            email: user.email,
            password: enc_password,
            encryption_salt: pwd_salt
        };

        let auth: UserForAuth = Self::create(dam, user_for_create).await?;
        
        Ok(auth)
    }

    pub async fn login(dam: &DataAccessManager, credentials: UserForLogin) -> Result<UserForAuth> {
        let UserForLogin { email, password } = credentials;
        let validation_fields = UserForValidate::get_struct_fields().join(", ");
        let query = format!("SELECT {} FROM {} WHERE email = $1", validation_fields ,Self::TABLE_NAME);
        let connection = dam.connect();

        let users_with_email: Vec<UserForValidate> = sqlx::query_as(&query)
        .bind(email)
        .fetch_all(connection)
        .await
        .map_err(|e| Error::QueryFailed(e))?;
        
        // little bit lengthy but it helps users know if it is the email or the password that is incorrect

        if users_with_email.is_empty() {
            return Err(Error::EntityNotFound);
        }

        // checks if the password provided encrypted with the password encryption salt is the same as the users password
        for user in users_with_email {
            let salt_string = user.encryption_salt.to_string();
            let enc_content = EncryptionContent {
                content: password.clone(),
                salt: str_to_base_64(&salt_string)
            };

            match validate_password(user.password, &enc_content) {
                Ok(()) => return Ok(UserForAuth { id: user.id, encryption_salt: user.encryption_salt}),
                Err(Error::PasswordInvalid) => continue,
                Err(e) => return Err(e)
            }
        }

        Err(Error::PasswordInvalid)
    }

    pub async fn get<T>(dam: &DataAccessManager, id: Uuid) -> Result<T> 
    where T: GettableUser {
        let user  = base_crud::get::<Self, T>(dam, id).await?;
        Ok(user)
    }

    async fn create<T>(dam: &DataAccessManager, user: UserForCreate) -> Result<T> 
    where T: GettableUser {
        let res = base_crud::create::<Self, UserForCreate, T>(dam, user).await?;
        Ok(res)
    }

    pub async fn delete(dam: &DataAccessManager, id: Uuid) -> Result<()> {
        base_crud::delete::<Self>(dam, id).await
    }
}