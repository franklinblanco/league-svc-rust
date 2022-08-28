use actix_web::http::StatusCode;
use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource};
use dev_communicators::middleware::{user_svc::user_service::{create_user, self}};
use dev_dtos::{domain::user::{credential_type::CredentialType, token::Token}, dtos::user::user_dtos::{UserForCreationDto, UserForAuthenticationDto}};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player_dtos::{PlayerForCreationDto, PlayerForUpdateDto}, util::{env_util::APP_NAME, error_util::handle_status_code_error_only}, dao::player_dao, domain::player::Player};


pub async fn create_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForCreationDto) -> TypedHttpResponse<Token> {
    let user_for_creation = UserForCreationDto {
        app: APP_NAME.to_string(),
        credential: player.phone_number.clone(),
        credential_type: CredentialType::PhoneNumber,
        password: player.password.clone(),
        name: player.name.clone()
    };

    let persisted_token = match create_user(client, &user_for_creation).await {
        Ok(user) => user,
        Err(err) => { return handle_status_code_error_only(err) },
    };

    match player_dao::insert_player(conn, Player::new_from_creation_dto(&player, &persisted_token.user_id)).await {
        Ok(_) => TypedHttpResponse::return_standard_response(StatusCode::OK, persisted_token),
        Err(err) => TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(err.to_string()))
    }
}
pub async fn edit_player_profile(_conn: &MySqlPool, client: &Client, player: PlayerForUpdateDto) -> TypedHttpResponse<Player> {
    let _user = match user_service::authenticate_user_with_token(client, &UserForAuthenticationDto { app: APP_NAME.to_string(), id: player.user_id.to_string(), token: player.auth_token }).await {
        Ok(user) => user,
        Err(err) => return handle_status_code_error_only(err)
    };



    return TypedHttpResponse::return_empty_response(StatusCode::OK);

    
    //TODO: Check for any of the fields having updates
    //TODO: Check for existing user
}