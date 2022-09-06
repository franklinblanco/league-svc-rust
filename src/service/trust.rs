use actix_web_utils::{extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error, dtos::message::MessageResource};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{domain::{trust::Trust}, dto::trust::{TrustRequestDto}, util::env_util::APP_NAME, dao::{player_dao::get_player_with_id, trust_dao}};


pub async fn add_trusted_player(conn: &MySqlPool, client: &Client, trust_req: TrustRequestDto) -> TypedHttpResponse<Trust> {
    let user_for_auth = UserForAuthenticationDto {app: APP_NAME.to_string(), id: trust_req.truster_id.to_string(), token: trust_req.auth_token.clone() };
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Trust
    );
    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as u32).await, Trust) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Truster Player profile not found.")),
    };
    match unwrap_or_return_handled_error!(get_player_with_id(conn, trust_req.trustee_id).await, Trust) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Trustee Player profile not found.")),
    };
    let trust_to_insert = Trust::new_from_join_request(&trust_req);
    if trust_req.truster_id == trust_req.trustee_id { return TypedHttpResponse::return_standard_error(400, MessageResource::new_from_str("You can't trust yourself..."))}
    unwrap_or_return_handled_error!(trust_dao::insert_trust(conn, &trust_to_insert).await, Trust);
    TypedHttpResponse::return_standard_response(200, trust_to_insert)
}

pub async fn remove_trusted_player(conn: &MySqlPool, client: &Client, trust_req: TrustRequestDto) -> TypedHttpResponse<Trust> {
    let user_for_auth = UserForAuthenticationDto {app: APP_NAME.to_string(), id: trust_req.truster_id.to_string(), token: trust_req.auth_token.clone() };
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Trust
    );
    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as u32).await, Trust) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Truster Player profile not found.")),
    };
    match unwrap_or_return_handled_error!(get_player_with_id(conn, trust_req.trustee_id).await, Trust) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Trustee Player profile not found.")),
    };
    match unwrap_or_return_handled_error!(trust_dao::delete_trust_with_both_ids(conn, trust_req.truster_id, trust_req.trustee_id).await, Trust).rows_affected() {
        0 => TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("You didn't trust this player in the first place.")),
        _ => TypedHttpResponse::return_empty_response(200)
    }
}

