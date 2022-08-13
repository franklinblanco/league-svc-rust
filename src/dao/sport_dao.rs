use sqlx::{MySqlPool, mysql::MySqlQueryResult};

use crate::domain::{sport::Sport};

pub async fn insert_sport(conn: &MySqlPool, sport: Sport) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query_file!("sql/sport/insert.sql", sport.name, sport.category_id).execute(conn).await
}

pub async fn get_sport_with_id(conn: &MySqlPool, sport_id: i32) -> Result<Option<Sport>, sqlx::Error>{
    sqlx::query_file_as!(Sport, "sql/sport/get.sql", sport_id).fetch_optional(conn).await
}

pub async fn update_sport_with_id(conn: &MySqlPool, sport: Sport) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/sport/update.sql", sport.name, sport.category_id, sport.id).execute(conn).await
}