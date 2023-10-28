use actix_web::HttpRequest;
use actix_web_utils::TypedResponse;
use err::{Error, trace};
use serde::Serialize;
use sqlx::PgPool;
use user_lib::{service::user::authenticate_user, dto::token::AuthenticateUserDto};

pub async fn authenticate_user_for_route<T: Serialize>(conn: &PgPool, request: HttpRequest) -> Result<i32, TypedResponse<T>> {
    let mut database_connection = match conn.acquire().await {
        Ok(connection) => connection,
        Err(error) => {
            return Err(TypedResponse::std_error(500, Error::new(trace!()).error_type(err::ErrorType::Service { error: err::ServiceError::DatabaseError { error }}).message("Error getting database connection from pool in authentication function.")));
        },
    };
    let error_to_return = TypedResponse::std_error(401, Error::new(trace!()).error_type(err::ErrorType::Privilege).message("authHeader not present or is in incorrect format. Make sure it comes as a AuthenticateUserDto Json"));
    let parsed_header: AuthenticateUserDto = match request.headers().get("authHeader") {
        Some(header) => match header.to_str() {
            Ok(header_str) => match serde_json::from_str(header_str) {
                Ok(parsed_header) => parsed_header,
                Err(error) => {
                    log::error!("{error}");
                    return Err(error_to_return);
                },
            },
            Err(error) => {
                log::error!("{error}");
                return Err(error_to_return);
            },
        },
        None => {
            return Err(error_to_return);
        },
    };
    let user_result = authenticate_user(&mut database_connection, parsed_header).await;
    match user_result {
        Ok(user) => Ok(user.id),
        Err(error) => {
            log::error!("{error}");
            Err(TypedResponse::std_error(401, Error::new(trace!()).error_type(err::ErrorType::Privilege).message("User Id and token combination not found in database.")))
        },
    }
}

#[macro_export]
macro_rules! authenticate {
    ($req:expr, $conn:expr) => {
        match crate::util::authentication::authenticate_user_for_route($conn, $req).await {
            Ok(user_id) => user_id,
            Err(error_response) => return error_response
        }
    };
}