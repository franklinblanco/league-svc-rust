use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::field::Field;
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_field(
    conn: &PgPool,
    field: Field,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/field/insert.sql",
            field.place_id,
            field.time_created,
            field.country,
            field.city,
            field.name,
            field.price_per_hour,
            field.currency,
            field.description
        )
        .execute(conn)
        .await
    )
}

pub async fn get_field_with_id(
    conn: &PgPool,
    field_id: i32,
) -> Result<Option<Field>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Field, "sql/field/get.sql", field_id)
            .fetch_optional(conn)
            .await
    )
}

pub async fn update_field_with_id(
    conn: &PgPool,
    field: Field,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/field/update.sql",
            field.place_id,
            field.last_updated,
            field.country,
            field.city,
            field.name,
            field.price_per_hour,
            field.currency,
            field.description,
            field.id
        )
        .execute(conn)
        .await
    )
}
