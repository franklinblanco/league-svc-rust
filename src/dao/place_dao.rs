use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::place::Place;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_place(
    conn: &PgPool,
    place: Place,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/place/insert.sql",
            place.time_created,
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
    conn: &PgPool,
    place_id: i32,
) -> Result<Option<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Place, "sql/place/get.sql", place_id)
            .fetch_optional(conn)
            .await
    )
}

pub async fn update_place_with_id(
    conn: &PgPool,
    place: Place,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/place/update.sql",
            place.last_updated,
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
    conn: &PgPool,
    country: String,
    page: i64,
) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            Place,
            "sql/place/get_by_country.sql",
            country,
            offset
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn get_place_with_sport_id_paged(
    conn: &PgPool,
    sport_id: i32,
    page: i64
) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            Place,
            "sql/place/get_by_sport_id.sql",
            sport_id,
            offset
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn get_all_places(conn: &PgPool) -> Result<Vec<Place>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Place, "sql/place/get_all.sql")
            .fetch_all(conn)
            .await
    )
}
