use league_types::domain::{dao_utils::Count, trust::Trust};
use sqlx::PgConnection;

pub async fn insert_trust(
    conn: &mut PgConnection,
    trust: &Trust,
) -> Result<Trust, sqlx::Error> {
    sqlx::query_file_as!(
        Trust, "sql/trust/insert.sql", trust.truster_id, trust.trustee_id, trust.time_created)
        .fetch_one(conn)
        .await
}

pub async fn get_trust_with_both_ids(
    conn: &mut PgConnection,
    truster_id: i32,
    trustee_id: i32,
) -> Result<Option<Trust>, sqlx::Error> {
    sqlx::query_file_as!(Trust, "sql/trust/get.sql", truster_id, trustee_id)
        .fetch_optional(conn)
        .await
}
//TODO: Check that fetch_one retreived a correct count from db
pub async fn get_trusts_by_truster_id(
    conn: &mut PgConnection,
    truster_id: i32,
) -> Result<Count, sqlx::Error> {
    sqlx::query_file_as!(Count, "sql/trust/get_count_by_truster.sql", truster_id)
        .fetch_one(conn)
        .await
}
//TODO: Check that fetch_one retreived a correct count from db
pub async fn get_trusts_by_trustee_id(
    conn: &mut PgConnection,
    trustee_id: i32,
) -> Result<Count, sqlx::Error> {
    sqlx::query_file_as!(Count, "sql/trust/get_count_by_trustee.sql", trustee_id)
        .fetch_one(conn)
        .await
}

pub async fn delete_trust_with_both_ids(
    conn: &mut PgConnection,
    truster_id: i32,
    trustee_id: i32,
) -> Result<Trust, sqlx::Error> {
    sqlx::query_file_as!(Trust, "sql/trust/delete.sql", truster_id, trustee_id)
        .fetch_one(conn)
        .await
}
