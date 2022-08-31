use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource, unwrap_or_return_handled_error};
use dev_communicators::middleware::{user_svc::user_service::{create_user, self}};
use dev_dtos::{domain::user::{credential_type::CredentialType, token::Token}, dtos::user::user_dtos::{UserForCreationDto, UserForAuthenticationDto}};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player_dtos::{PlayerForCreationDto, PlayerForUpdateDto}, util::{env_util::APP_NAME, converter}, dao::player_dao, domain::player::Player};

pub async fn create_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForCreationDto) -> TypedHttpResponse<Token> {
    let user_for_creation = UserForCreationDto {
        app: APP_NAME.to_string(),
        credential: player.phone_number.clone(),
        credential_type: CredentialType::PhoneNumber,
        password: player.password.clone(),
        name: player.name.clone()
    };

    let persisted_token = unwrap_or_return_handled_error!(create_user(client, &user_for_creation).await, Token);

    match player_dao::insert_player(conn, Player::new_from_creation_dto(&player, &persisted_token.user_id)).await {
        Ok(_) => TypedHttpResponse::return_standard_response(200, persisted_token),
        Err(err) => TypedHttpResponse::return_standard_error(500, MessageResource::new_from_err(err.error.to_string()))
    }
}

pub async fn edit_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForUpdateDto) -> TypedHttpResponse<Player> {
    let persisted_user = unwrap_or_return_handled_error!(user_service::authenticate_user_with_token(client, &UserForAuthenticationDto { app: APP_NAME.to_string(), id: player.user_id.to_string(), token: player.auth_token.clone() }).await, Player);
    //  Attempt to find player in database with the user id that user service gave back

    let persisted_player = match unwrap_or_return_handled_error!(player_dao::get_player_with_id(conn, persisted_user.id).await, Player) {
            Some(found_player) => found_player,
            None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Could not find player with id. Something went wrong.")),
        };

    //  Attempt to apply the updates and pass the validation for each player property
    let player_to_update = unwrap_or_return_handled_error!(400, converter::update_player_struct(player, persisted_player), Player);

    //  Apply changes in the database.
    unwrap_or_return_handled_error!(player_dao::update_player_with_id(conn, player_to_update).await, Player);

    //  Debating in between an empty response with an OK or a more elaborate response with the updated Player.
    return TypedHttpResponse::return_empty_response(200);
}