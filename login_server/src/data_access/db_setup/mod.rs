#[cfg(test)]
mod tests;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::{Error, Result};
use crate::get_env::get_env_static;
use std::fs;
use std::time::Duration;
use tracing::info;


pub async fn make_migrations(pool: &Pool<Postgres>) -> Result<()> {
    
    sqlx::migrate!("./db/sql/migrations")
    .run(pool)
    .await
    .map_err(|e| Error::MigrationFailed(e))?;

    info!("created migrations");

    Ok(())
}


pub async fn _reset_db() -> Result<()> {
    let connect_as_default_user = &get_env_static().DB_DEFAULT_USER_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_default_user).await?;

    let file_data  = fs::read_to_string("./db/sql/recreate_db.sql")   
    .map_err(|e| Error::FailedToReadFiles(format!("{} \n\nA common fault is to not run this command from the root directory /backend", e)))?;
    
    let recreation_commands: Vec<&str> = file_data.split(";").collect();

    for command in recreation_commands { 
        sqlx::query(command)
        .execute(&pool)
        .await
        .map_err(|e| Error::QueryFailed(e))?;
    }

    info!("reset db");

    Ok(())
}

pub async fn create_connection_pool(connection_string: &str) -> Result<Pool<Postgres>> {
    let pool =PgPoolOptions::new()
    .max_connections(30) // if pool times out in tests, increase this
    .acquire_timeout(Duration::from_secs(60)) // or this
    .connect(connection_string)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    info!("created connection pool");

    Ok(pool)
}


pub async fn create_service_user_connection_pool() -> Result<Pool<Postgres>> {
    let connect_as_service_user = &get_env_static().DB_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_service_user).await?;

    Ok(pool)
}
