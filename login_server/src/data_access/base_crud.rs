
use std::collections::btree_map::Keys;

use crate::{
    utils::traits::{
        AsHashMap, 
        FieldsAsStrings
    }, 
    Error, 
    Result
};
use sqlx::{postgres::PgRow, FromRow};
use uuid::Uuid;
use super::DataAccessManager;


pub trait Controller {
    const TABLE_NAME: &'static str;
}

pub async fn get<C, T>(dam: &DataAccessManager, id: Uuid) ->  Result<T>
where 
    C: Controller,
    T: for<'a> FromRow<'a, PgRow> + Send + Unpin + FieldsAsStrings 
{
    let connection = dam.connect();
    let struct_fields_string = T::get_struct_fields().join(", ");
    let sql = format!("SELECT {} FROM {} WHERE id = $1", struct_fields_string, C::TABLE_NAME);
    
    let row: T = sqlx::query_as(&sql)
    .bind(id)
    .fetch_one(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(row)
}

pub async fn create<C, T>(dam: &DataAccessManager, data: T) -> Result<Uuid>
where
    C: Controller,
    T: AsHashMap,
{
    let connection = dam.connect();
    let hashmap = data.to_hashmap();
    let keys: Vec<_> = hashmap.keys().into_iter().map(|k| k.to_string()).collect();
    let values: Vec<_> = hashmap.values().into_iter().map(|v| format!("'{}'", v)).collect();

    let sql = format!("INSERT INTO {} ({}) VALUES ({}) RETURNING id", C::TABLE_NAME, keys.join(", "), values.join(", "));

    let (id,) = sqlx::query_as::<_, (Uuid,)>(&sql)
    .fetch_one(connection)
    .await
    .map_err(|e| Error::QueryFailed(e))?;

    Ok(id)
}

pub async fn delete<C>(dam: &DataAccessManager, id: Uuid) -> Result<()> where C: Controller {
    let connection = dam.connect();
    let sql = format!("DELETE FROM {} WHERE id = $1", C::TABLE_NAME);

    let rows_affected = sqlx::query(&sql)
    .bind(id)
    .execute(connection)
    .await
    .map_err(|e: sqlx::Error| Error::QueryFailed(e))?
    .rows_affected();

    if rows_affected == 0 {
        return Err(Error::EntityNotFound);
    }

    Ok(())
}