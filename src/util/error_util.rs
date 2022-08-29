use actix_web::http::StatusCode;
use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource};
use dev_dtos::enums::error::Error;
use serde::Serialize;

/// This is a little counter intuitive, as there is many error types I have defined. For some cases I don't really care what error it is,
/// Just that it returns the status code it got, usually use this when calling a communicator method and it doesn't return what I expected.
pub fn handle_status_code_error_only<T: Serialize>(error: Error) -> TypedHttpResponse<T> {
    match error {
        Error::UnexpectedStatusCode(_, actual, errorstr) => TypedHttpResponse::return_standard_error(StatusCode::from_u16(actual).unwrap(), MessageResource::new_from_err(errorstr)),
        _ => TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(error.to_string()))
    }
}
/// Method used to avoid typing this verbose ass error, just yeet the error back to the client With an internal server error. (NOTE: Should not be used for NOT_FOUND)
/// This is to be used when soemthing goes wrong in the database. Not an expected behaviour.
pub fn handle_database_error<T: Serialize>(error: sqlx::Error) -> TypedHttpResponse<T> {
    TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(error.to_string()))
}