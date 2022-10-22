use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::place::Place;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

pub async fn insert_place(
    conn: &MySqlPool,
    place: Place,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/place/insert.sql",
            place.name,
            place.sport_id,
            place.country,
            place.state,
            place.city,
            place.address,
            place.maps_url,
            place.contact_number,
            place.picture_url
        )
        .execute(conn)
        .await
    )
}

pub async fn get_place_with_id(
    conn: &MySqlPool,
    place_id: u32,
) -> Result<Option<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Place, "sql/place/get.sql", place_id)
            .fetch_optional(conn)
            .await
    )
}

pub async fn update_place_with_id(
    conn: &MySqlPool,
    place: Place,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/place/update.sql",
            place.name,
            place.sport_id,
            place.country,
            place.state,
            place.city,
            place.address,
            place.maps_url,
            place.contact_number,
            place.picture_url,
            place.id
        )
        .execute(conn)
        .await
    )
}

pub async fn get_places_with_country_paged(
    conn: &MySqlPool,
    country: String,
    from_row: u32,
    to_row: u32,
) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            Place,
            "sql/place/get_by_country.sql",
            country,
            from_row,
            to_row
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn get_place_with_sport_id_paged(
    conn: &MySqlPool,
    sport_id: u32,
    from_row: u32,
    to_row: u32,
) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            Place,
            "sql/place/get_by_sport_id.sql",
            sport_id,
            from_row,
            to_row
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn get_all_places(conn: &MySqlPool) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Place, "sql/place/get_all.sql")
            .fetch_all(conn)
            .await
    )
}
