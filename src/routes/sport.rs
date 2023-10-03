use std::sync::Arc;

use actix_web::{get, web::Data};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::domain::sport::Sport;
use sqlx::PgPool;

use crate::{create_tx, service};

#[get("")]
pub async fn get_all_sports(conn: Data<Arc<PgPool>>) -> TypedResponse<Vec<Sport>> {
    let mut transaction = create_tx!(conn);
    service::sport::get_all_sports(&mut *transaction).await.to_response()
}
