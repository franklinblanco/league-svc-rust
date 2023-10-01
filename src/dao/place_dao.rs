use league_types::domain::place::Place;
use sqlx::PgConnection;

pub async fn insert_place(
    conn: &mut PgConnection,
    place: Place,
) -> Result<Place, sqlx::Error> {
    sqlx::query_file_as!(
        Place,
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
    .fetch_one(conn)
    .await
}

pub async fn get_place_with_id(
    conn: &mut PgConnection,
    place_id: i32,
) -> Result<Option<Place>, sqlx::Error> {
    sqlx::query_file_as!(Place, "sql/place/get.sql", place_id)
        .fetch_optional(conn)
        .await
}

pub async fn update_place_with_id(
    conn: &mut PgConnection,
    place: Place,
) -> Result<Place, sqlx::Error> {
    sqlx::query_file_as!(
        Place,
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
    .fetch_one(conn)
    .await
}

pub async fn get_places_with_country_paged(
    conn: &mut PgConnection,
    country: String,
    page: i64,
) -> Result<Vec<Place>, sqlx::Error> {
    let offset = (page - 1) * 25;
    sqlx::query_file_as!(
        Place,
        "sql/place/get_by_country.sql",
        country,
        offset
    )
    .fetch_all(conn)
    .await
}

pub async fn get_place_with_sport_id_paged(
    conn: &mut PgConnection,
    sport_id: i32,
    page: i64
) -> Result<Vec<Place>, sqlx::Error> {
    let offset = (page - 1) * 25;
    sqlx::query_file_as!(
        Place,
        "sql/place/get_by_sport_id.sql",
        sport_id,
        offset
    )
    .fetch_all(conn)
    .await
}

pub async fn get_all_places(conn: &mut PgConnection,) -> Result<Vec<Place>, sqlx::Error> {
    sqlx::query_file_as!(Place, "sql/place/get_all.sql")
        .fetch_all(conn)
        .await
}
