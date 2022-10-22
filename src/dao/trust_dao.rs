use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::{dao_utils::Count, trust::Trust};
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

pub async fn insert_trust(
    conn: &MySqlPool,
    trust: &Trust,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!("sql/trust/insert.sql", trust.truster_id, trust.trustee_id)
            .execute(conn)
            .await
    )
}

pub async fn get_trust_with_both_ids(
    conn: &MySqlPool,
    truster_id: u32,
    trustee_id: u32,
) -> Result<Option<Trust>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Trust, "sql/trust/get.sql", truster_id, trustee_id)
            .fetch_optional(conn)
            .await
    )
}
//TODO: Check that fetch_one retreived a correct count from db
pub async fn get_trusts_by_truster_id(
    conn: &MySqlPool,
    truster_id: u32,
) -> Result<Count, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Count, "sql/trust/get_count_by_truster.sql", truster_id)
            .fetch_one(conn)
            .await
    )
}
//TODO: Check that fetch_one retreived a correct count from db
pub async fn get_trusts_by_trustee_id(
    conn: &MySqlPool,
    trustee_id: u32,
) -> Result<Count, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(Count, "sql/trust/get_count_by_trustee.sql", trustee_id)
            .fetch_one(conn)
            .await
    )
}

pub async fn delete_trust_with_both_ids(
    conn: &MySqlPool,
    truster_id: u32,
    trustee_id: u32,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!("sql/trust/delete.sql", truster_id, trustee_id)
            .execute(conn)
            .await
    )
}
