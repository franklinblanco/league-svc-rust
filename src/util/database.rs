

#[macro_export]
macro_rules! create_tx {
    ( $conn:expr ) => {
        match $conn.begin().await {
            Ok(tx) => tx,
            Err(error) => {
                log::error!("{error}");
                return actix_web_utils::TypedResponse::std_error(500, err::Error::new(err::trace!()).error_type(err::ErrorType::Service(err::ServiceError::DatabaseError(error))).message("Error getting database transaction from pool in pre-service-macro"));
            },
        }
    };
}

#[macro_export]
macro_rules! finish_tx {
    ( $service_response:expr, $tx:expr ) => {
        match $service_response {
            Ok(_) => {
                let _ = $tx.commit().await;
                $service_response.to_response()
            },
            Err(_) => {
                let _ = $tx.rollback().await;
                $service_response.to_response()
            }
        }
    };
}