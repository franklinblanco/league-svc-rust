use actix_web::http::StatusCode;
use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource};
use dev_dtos::enums::error::Error;
use serde::Serialize;

pub fn handle_status_code_error_only<T: Serialize>(error: Error) -> TypedHttpResponse<T> {
    match error {
        Error::UnexpectedStatusCode(_, actual, errorstr) => return TypedHttpResponse::return_standard_error(StatusCode::from_u16(actual).unwrap(), MessageResource::new_from_err(errorstr)),
        _ => return TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(error.to_string()))
    };
}