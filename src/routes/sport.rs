use std::sync::Arc;

use actix_web::{get, web::Data};
use actix_web_utils::{unwrap_or_return_handled_error, extensions::typed_response::TypedHttpResponse};
use sqlx::MySqlPool;
use league_types::domain::sport::Sport;

use crate::dao::sport_dao;


#[get("")]
pub async fn get_all_sports(conn: Data<Arc<MySqlPool>>) -> TypedHttpResponse<Vec<Sport>> {
    unwrap_or_return_handled_error!(500, 200, sport_dao::get_all_sports_ordered(&conn).await, Vec<Sport>)
}