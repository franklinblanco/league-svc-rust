use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use reqwest::StatusCode;


pub async fn create_league() -> TypedHttpResponse<String> {
    TypedHttpResponse::return_empty_response(StatusCode::OK)
}