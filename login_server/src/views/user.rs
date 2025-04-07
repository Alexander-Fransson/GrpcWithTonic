use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::utils::traits::{AsHashMap, FieldsAsStrings};
use std::collections::HashMap;


#[derive(FieldsAsStrings, FromRow, Debug)]
pub struct UserForGet {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(AsHashMap, Clone)]
pub struct UserForCreate {
    pub name: String,
    pub email: String,
    pub password: String
}