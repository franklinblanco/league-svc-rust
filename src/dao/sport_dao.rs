use actix_web_utils::{wrap_generic_error_in_wrapper, extensions::generic_error::GenericError};
use league_types::domain::sport::Sport;
use sqlx::{MySqlPool, mysql::MySqlQueryResult, Transaction, MySql};

pub async fn insert_sport(conn: &mut Transaction<'_, MySql>, sport: Sport) -> Result<MySqlQueryResult, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/sport/insert.sql", sport.id, sport.name, sport.category_id).execute(conn).await)
}

pub async fn get_sport_with_id(conn: &MySqlPool, sport_id: u32) -> Result<Option<Sport>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Sport, "sql/sport/get.sql", sport_id).fetch_optional(conn).await)
}

pub async fn get_all_sports_ordered(conn: &MySqlPool) -> Result<Vec<Sport>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Sport, "sql/sport/get_all_ordered.sql").fetch_all(conn).await)
}


pub async fn update_sport_with_id(conn: &MySqlPool, sport: Sport) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/sport/update.sql", sport.name, sport.category_id, sport.id).execute(conn).await)
}