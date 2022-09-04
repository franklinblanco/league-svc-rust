use sqlx::{MySqlPool, mysql::MySqlQueryResult};
use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};

use crate::domain::{place::Place};

pub async fn insert_place(conn: &MySqlPool, place: Place) -> Result<MySqlQueryResult, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/place/insert.sql", place.name, place.sport_id, place.country, place.state, place.city, place.address, place.maps_url, place.contact_number, place.picture_url).execute(conn).await)
}

pub async fn get_place_with_id(conn: &MySqlPool, place_id: i32) -> Result<Option<Place>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Place, "sql/place/get.sql", place_id).fetch_optional(conn).await)
}

pub async fn update_place_with_id(conn: &MySqlPool, place: Place) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/place/update.sql", place.name, place.sport_id, place.country, place.state, place.city, place.address, place.maps_url, place.contact_number, place.picture_url, place.id).execute(conn).await)
}