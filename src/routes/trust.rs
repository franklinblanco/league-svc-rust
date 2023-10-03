use std::sync::Arc;

use actix_web::{
    delete, post,
    web::{Data, Json}, HttpRequest,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::{domain::trust::Trust, dto::trust::TrustRequestDto};
use sqlx::PgPool;

use crate::{service::trust, authenticate, create_tx, finish_tx};

#[post("")]
pub async fn add_trusted_player(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    trust_req: Json<TrustRequestDto>,
) -> TypedResponse<Trust> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = trust::add_trusted_player(&mut *transaction, trust_req.0, user_id).await;
    finish_tx!(response, transaction)
}

#[delete("")]
pub async fn remove_trusted_player(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    trust_req: Json<TrustRequestDto>,
) -> TypedResponse<Trust> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = trust::remove_trusted_player(&mut *transaction, trust_req.0, user_id).await;
    finish_tx!(response, transaction)
}
