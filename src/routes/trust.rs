use std::sync::Arc;

use actix_web::{post, web::{Json, Data}, delete};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use reqwest::Client;
use sqlx::MySqlPool;
use league_types::{domain::trust::Trust, dto::trust::TrustRequestDto};

use crate::service::trust;

#[post("")]
pub async fn add_trusted_player(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, trust_req: Json<TrustRequestDto>) -> TypedHttpResponse<Trust> {
    trust::add_trusted_player(&conn, &client, trust_req.0).await
}

#[delete("")]
pub async fn remove_trusted_player(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, trust_req: Json<TrustRequestDto>) -> TypedHttpResponse<Trust> {
    trust::remove_trusted_player(&conn, &client, trust_req.0).await
}
