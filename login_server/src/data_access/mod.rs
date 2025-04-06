mod db_setup;
mod base_crud;

use db_setup::{_reset_db, create_service_user_connection_pool, make_migrations};
use sqlx::{Pool, Postgres};
use crate::Result;
use tokio::sync::OnceCell;

#[derive(Clone)]
pub struct DataAccessManager {
    db_connection: Pool<Postgres>
}

impl DataAccessManager {
    pub async fn new() -> Result<Self> {
       let db_connection = create_service_user_connection_pool().await?;

       _reset_db().await?; // remvoe in production, lest it resets on every startup
       make_migrations(&db_connection).await?;

       Ok(Self {
           db_connection
       })
    }
}

pub async fn _get_data_access_manager_instance_for_tests() -> DataAccessManager {
    static DB: OnceCell<DataAccessManager> = OnceCell::const_new();

    let db = DB.get_or_init(|| async {
        DataAccessManager::new().await.unwrap()
    }).await;

    db.clone()
}