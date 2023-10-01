use league_types::domain::sport::Sport;
use sqlx::PgConnection;

pub async fn insert_sport(
    conn: &mut PgConnection,
    sport: Sport,
) -> Result<Sport, sqlx::Error> {
    sqlx::query_file_as!(
        Sport,
        "sql/sport/insert.sql",
        sport.id,
        sport.name,
        sport.category_id
    )
    .fetch_one(conn)
    .await
}

pub async fn get_sport_with_id(
    conn: &mut PgConnection,
    sport_id: i32,
) -> Result<Option<Sport>, sqlx::Error> {
    sqlx::query_file_as!(Sport, "sql/sport/get.sql", sport_id)
        .fetch_optional(conn)
        .await
}

pub async fn get_all_sports_ordered(
    conn: &mut PgConnection,
) -> Result<Vec<Sport>, sqlx::Error> {
    sqlx::query_file_as!(Sport, "sql/sport/get_all_ordered.sql")
        .fetch_all(conn)
        .await
}

pub async fn update_sport_with_id(
    conn: &mut PgConnection,
    sport: Sport,
) -> Result<Sport, sqlx::Error> {
    sqlx::query_file_as!(
        Sport,
        "sql/sport/update.sql",
        sport.name,
        sport.category_id,
        sport.id
    )
    .fetch_one(conn)
    .await
}
