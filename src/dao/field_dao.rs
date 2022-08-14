use sqlx::{MySqlPool, mysql::MySqlQueryResult};

use crate::domain::{field::Field};

pub async fn insert_field(conn: &MySqlPool, field: Field) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query_file!("sql/field/insert.sql", field.place_id, field.country, field.city, field.name, field.price_per_hour, field.currency, field.description).execute(conn).await
}

pub async fn get_field_with_id(conn: &MySqlPool, field_id: i32) -> Result<Option<Field>, sqlx::Error>{
    sqlx::query_file_as!(Field, "sql/field/get.sql", field_id).fetch_optional(conn).await
}

pub async fn update_field_with_id(conn: &MySqlPool, field: Field) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/field/update.sql", field.place_id, field.country, field.city, field.name, field.price_per_hour, field.currency, field.description, field.id).execute(conn).await
}