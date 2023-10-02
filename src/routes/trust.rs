use std::sync::Arc;

use actix_web::{
    delete, post,
    web::{Data, Json}, HttpRequest,
};
use actix_web_utils::extensions::typed_response::TypedResponse;
use league_types::{domain::trust::Trust, dto::trust::TrustRequestDto};
use reqwest::Client;
use sqlx::PgPool;

use crate::{service::trust, authenticate};

#[post("")]
pub async fn add_trusted_player(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    trust_req: Json<TrustRequestDto>,
) -> TypedResponse<Trust> {
    let user_id = authenticate!(request, &conn);
    trust::add_trusted_player(&conn, &client, trust_req.0).await.to_response()
}

#[delete("")]
pub async fn remove_trusted_player(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    trust_req: Json<TrustRequestDto>,
) -> TypedResponse<Trust> {
    let user_id = authenticate!(request, &conn);
    trust::remove_trusted_player(&conn, &client, trust_req.0).await.to_response()
}
