use sqlx::{postgres::PgRow, prelude::FromRow};
use uuid::Uuid;

use crate::utils::traits::{AsHashMap, FieldsAsStrings};
use std::collections::HashMap;

pub trait GettableUser: for <'a> FromRow<'a, PgRow> + Unpin + Send + FieldsAsStrings {}

#[derive(FieldsAsStrings, FromRow, Debug)]
pub struct UserForAuth {
    pub id: Uuid,
    pub encryption_salt: Uuid
}

#[derive(FieldsAsStrings, FromRow, Debug)]
pub struct UserForGet {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(FieldsAsStrings, FromRow, Debug)]
pub struct UserForValidate {
    pub id: Uuid,
    pub password: String,
    pub encryption_salt: Uuid
}

#[allow(dead_code)]
// this is not used in the main code
#[derive(FieldsAsStrings, FromRow, Debug)]
pub struct FullUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub encryption_salt: Uuid
}

impl GettableUser for UserForGet {}
impl GettableUser for UserForAuth {}
impl GettableUser for FullUser {}

#[derive(AsHashMap, Clone)]
pub struct UserForCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub encryption_salt: Uuid
}

pub struct UserForRegister {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserForLogin {
    pub email: String,
    pub password: String
}

