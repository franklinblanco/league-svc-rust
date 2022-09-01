use actix_web_utils::{unwrap_or_return_handled_error, extensions::typed_response::TypedHttpResponse};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dao::sport_dao, domain::sport::Sport};

pub async fn get_all_sports(conn: &MySqlPool, _client: &Client,) -> TypedHttpResponse<Vec<Sport>> {
    unwrap_or_return_handled_error!(500, 200, sport_dao::get_all_sports_ordered(conn).await, Vec<Sport>)
}