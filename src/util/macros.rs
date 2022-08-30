
pub mod macros {
    /// This is to minimize the amount of unwraps made in the code
    /// Give it a Result<Whatever_you_want_to_return, Error> and it'll
    /// Basically unwrap the result if its there and if it isn't it'll return an error.
    /// No need for repetitive match statements!
    macro_rules! unwrap_or_return_handled_error {
        ( $e:expr ) => {
            match $e {
                Ok(value) => value,
                Err(error) => match error {
                    dev_dtos::enums::error::Error::UnexpectedStatusCode(_, actual, errorstr) => return TypedHttpResponse::return_standard_error(StatusCode::from_u16(actual).unwrap(), MessageResource::new_from_err(errorstr)),
                    _ => return TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(error.to_string()))
                },
            }
        }
    }
    pub(crate) use unwrap_or_return_handled_error;
}
