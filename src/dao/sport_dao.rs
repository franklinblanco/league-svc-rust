use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::sport::Sport;
use sqlx::{postgres::PgQueryResult, PgPool, Transaction, Postgres};

pub async fn insert_sport(
    conn: &mut Transaction<'_, Postgres>,
    sport: Sport,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/sport/insert.sql",
            sport.id,
            sport.name,
            sport.category_id
        )
        .execute(conn)
        .await
    )
}

pub async fn get_sport_with_id(
    conn: &PgPool,
    sport_id: i32,
) -> Result<Option<Sport>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Sport, "sql/sport/get.sql", sport_id)
            .fetch_optional(conn)
            .await
    )
}

pub async fn get_all_sports_ordered(
    conn: &PgPool,
) -> Result<Vec<Sport>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Sport, "sql/sport/get_all_ordered.sql")
            .fetch_all(conn)
            .await
    )
}

pub async fn update_sport_with_id(
    conn: &PgPool,
    sport: Sport,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/sport/update.sql",
            sport.name,
            sport.category_id,
            sport.id
        )
        .execute(conn)
        .await
    )
}
