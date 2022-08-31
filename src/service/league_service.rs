use actix_web_utils::extensions::typed_response::TypedHttpResponse;


pub async fn create_league() -> TypedHttpResponse<String> {
    TypedHttpResponse::return_empty_response(200)
}