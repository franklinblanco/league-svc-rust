use actix_web::http::StatusCode;
use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource};
use dev_communicators::middleware::{user_svc::user_service::{create_user, self}};
use dev_dtos::{domain::user::{credential_type::CredentialType, token::Token}, dtos::user::user_dtos::{UserForCreationDto, UserForAuthenticationDto}};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player_dtos::{PlayerForCreationDto, PlayerForUpdateDto}, util::{env_util::APP_NAME, error_util::{handle_status_code_error_only, handle_database_error}, converter}, dao::player_dao, domain::player::Player};


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

pub async fn edit_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForUpdateDto) -> TypedHttpResponse<Player> {
    let persisted_user = match user_service::authenticate_user_with_token(client, &UserForAuthenticationDto { app: APP_NAME.to_string(), id: player.user_id.to_string(), token: player.auth_token.clone() }).await {
        Ok(user) => user,
        Err(err) => return handle_status_code_error_only(err)
    };
    //  Attempt to find player in database with the user id that user service gave back.
    let persisted_player = match player_dao::get_player_with_id(conn, persisted_user.id).await {
        Ok(result) => match result {
            Some(found_player) => found_player,
            None => return TypedHttpResponse::return_standard_error(StatusCode::NOT_FOUND, MessageResource::new_from_str("Could not find player with id. Something went wrong.")),
        },
        Err(err) => return handle_database_error(err),
    };

    //  Attempt to apply the updates and pass the validation for each player property
    let player_to_update = match converter::update_player_struct(player, persisted_player) {
        Ok(updated_player) => updated_player,
        Err(messages) => return TypedHttpResponse::return_standard_error_list(StatusCode::BAD_REQUEST, messages)
    };

    //  Apply changes in the database.
    match player_dao::update_player_with_id(conn, player_to_update).await {
        Ok(_) => {},
        Err(err) => return handle_database_error(err),
    };

    //  Debating in between an empty response with an OK or a more elaborate response with the updated Player.
    return TypedHttpResponse::return_empty_response(StatusCode::OK);
}