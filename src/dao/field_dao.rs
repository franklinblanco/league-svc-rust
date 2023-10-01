use league_types::domain::field::Field;
use sqlx::PgConnection;

pub async fn insert_field(
    conn: &mut PgConnection,
    field: Field,
) -> Result<Field, sqlx::Error> {
    
        sqlx::query_file_as!(
            Field,
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
        .fetch_one(conn)
        .await
}

pub async fn get_field_with_id(
    conn: &mut PgConnection,
    field_id: i32,
) -> Result<Option<Field>, sqlx::Error> {
    sqlx::query_file_as!(Field, "sql/field/get.sql", field_id)
        .fetch_optional(conn)
        .await
}

pub async fn update_field_with_id(
    conn: &mut PgConnection,
    field: Field,
) -> Result<Field, sqlx::Error> {
    sqlx::query_file_as!(
        Field,
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
    .fetch_one(conn)
    .await
}
