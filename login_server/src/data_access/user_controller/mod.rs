use uuid::Uuid;
use crate::Result;
use crate::views::user::{UserForCreate, UserForGet};
use super::{base_crud::{self, Controller}, DataAccessManager};

#[cfg(test)]
mod tests;

pub struct UserController;

impl Controller for UserController {
    const TABLE_NAME: &'static str = "\"user\"";
}

impl UserController {
    pub async fn get(dam: &DataAccessManager, id: Uuid) -> Result<UserForGet> {
        let user  = base_crud::get::<Self, UserForGet>(dam, id).await?;
        Ok(user)
    }

    pub async fn create(dam: &DataAccessManager, user: UserForCreate) -> Result<Uuid> {
        let id = base_crud::create::<Self, UserForCreate>(dam, user).await?;
        Ok(id)
    }

    pub async fn delete(dam: &DataAccessManager, id: Uuid) -> Result<()> {
        base_crud::delete::<Self>(dam, id).await
    }
}