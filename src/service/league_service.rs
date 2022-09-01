use actix_web_utils::extensions::typed_response::TypedHttpResponse;

use crate::domain::league::League;

pub async fn create_league() -> TypedHttpResponse<League> {
    TypedHttpResponse::return_empty_response(200)
}