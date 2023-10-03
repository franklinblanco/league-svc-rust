use actix_web_utils::{extensions::typed_response::TypedResponse, ServiceResponse};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use err::MessageResource;
use league_types::{domain::trust::Trust, dto::trust::TrustRequestDto};
use reqwest::Client;
use sqlx::{PgPool, PgConnection};

use crate::dao::{player_dao::get_player_with_id, trust_dao};

pub async fn add_trusted_player(
    conn: &mut PgConnection,
    client: &Client,
    trust_req: TrustRequestDto,
) -> ServiceResponse<Trust> {
    match unwrap_or_return_handled_error!(
        trust_dao::get_trust_with_both_ids(conn, trust_req.truster_id, trust_req.trustee_id).await,
        Trust
    ) {
        Some(_) => {
            return TypedResponse::return_standard_error(
                400,
                MessageResource::new_from_str("You already trust this player."),
            )
        }
        None => { /* Do nothing */ }
    };
    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as i32).await, Trust) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Truster Player profile not found."),
            )
        }
    };
    match unwrap_or_return_handled_error!(
        get_player_with_id(conn, trust_req.trustee_id).await,
        Trust
    ) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Trustee Player profile not found."),
            )
        }
    };
    let trust_to_insert = Trust::from(trust_req.clone());
    if trust_req.truster_id == trust_req.trustee_id {
        return TypedResponse::return_standard_error(
            400,
            MessageResource::new_from_str("You can't trust yourself..."),
        );
    }
    unwrap_or_return_handled_error!(trust_dao::insert_trust(conn, &trust_to_insert).await, Trust);
    TypedResponse::return_standard_response(200, trust_to_insert)
}

pub async fn remove_trusted_player(
    conn: &mut PgConnection,
    client: &Client,
    trust_req: TrustRequestDto,
) -> ServiceResponse<Trust> {
    let user_for_auth = UserForAuthenticationDto {
        app: APP_NAME.to_string(),
        id: trust_req.truster_id.to_string(),
        token: trust_req.auth_token.clone(),
    };
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Trust
    );
    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as i32).await, Trust) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Truster Player profile not found."),
            )
        }
    };
    match unwrap_or_return_handled_error!(
        get_player_with_id(conn, trust_req.trustee_id).await,
        Trust
    ) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Trustee Player profile not found."),
            )
        }
    };
    match unwrap_or_return_handled_error!(
        trust_dao::delete_trust_with_both_ids(conn, trust_req.truster_id, trust_req.trustee_id)
            .await,
        Trust
    )
    .rows_affected()
    {
        0 => TypedResponse::return_standard_error(
            404,
            MessageResource::new_from_str("You didn't trust this player in the first place."),
        ),
        _ => TypedResponse::return_empty_response(200),
    }
}
