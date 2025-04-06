use serial_test::serial;
use sqlx::Row; // needed to use .get on PgRow
use crate::get_env::get_env_static;
use crate::data_access::db_setup::{
    create_connection_pool,
    make_migrations,
    _reset_db,
    Result, 
    Error
};

#[serial]
#[tokio::test]
async fn test_create_connection_pool() -> Result<()> {
    let connect_as_default_user = &get_env_static().DB_DEFAULT_USER_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_default_user).await?;

    let test = sqlx::query("SELECT 1 + 1  AS sum")
    .fetch_one(&pool)
    .await
    .map_err(|e| Error::QueryFailed(e))?;


    assert_eq!(test.get::<i32, _>("sum"), 2);

    Ok(())
}

#[serial]
#[tokio::test]
async fn test_make_migrations() -> Result<()> {
    _reset_db().await?;

    let connect_as_service_user = &get_env_static().DB_CONNECTION_STRING;
    let pool = create_connection_pool(connect_as_service_user).await?;

    make_migrations(&pool).await?;

    let users = sqlx::query("SELECT * FROM \"user\"")
    .fetch_all(&pool)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    assert!(users.len() > 0);

    Ok(())
}