use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource, unwrap_or_return_handled_error};
use dev_communicators::middleware::{user_svc::user_service::{create_user, self}};
use dev_dtos::{domain::user::{token::Token}, dtos::user::user_dtos::{UserForAuthenticationDto, UserForLoginDto}};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player::{PlayerForCreationDto, PlayerForUpdateDto}, util::{env_util::APP_NAME, converter}, dao::player_dao, domain::player::Player};

/// Self explanatory name
pub async fn create_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForCreationDto) -> TypedHttpResponse<Token> {
    let user_for_creation = PlayerForCreationDto::convert_player_into_user_for_creation(&player);

    let persisted_token = unwrap_or_return_handled_error!(create_user(client, &user_for_creation).await, Token);
    let player_to_persist = Player::new_from_creation_dto(&player, &persisted_token.user_id);
    unwrap_or_return_handled_error!(500, player_dao::insert_player(conn, player_to_persist).await, Token);
    TypedHttpResponse::return_standard_response(200, persisted_token)
}
//TODO: Sign in & forgot password?

/// Called to update any detail in the player profile
pub async fn edit_player_profile(conn: &MySqlPool, client: &Client, player: PlayerForUpdateDto) -> TypedHttpResponse<Player> {
    let persisted_user = unwrap_or_return_handled_error!(user_service::authenticate_user_with_token(client, &UserForAuthenticationDto { app: APP_NAME.to_string(), id: player.user_id.to_string(), token: player.auth_token.clone() }).await, Player);
    //  Attempt to find player in database with the user id that user service gave back
    let persisted_player = match unwrap_or_return_handled_error!(player_dao::get_player_with_id(conn, persisted_user.id).await, Player) {
            Some(found_player) => found_player,
            None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Could not find player with id. Something went wrong.")),
        };
    let player_to_update = unwrap_or_return_handled_error!(400, converter::update_player_struct(player, persisted_player), Player);
    unwrap_or_return_handled_error!(player_dao::update_player_with_id(conn, player_to_update).await, Player);
    //  Debating in between an empty response with an OK or a more elaborate response with the updated Player.
    return TypedHttpResponse::return_empty_response(200);
}
//TODO: Verify user
pub async fn login(conn: &MySqlPool, client: &Client, mut user: UserForLoginDto) -> TypedHttpResponse<Player> {
    user.app = APP_NAME.to_string();
    let persisted_user = unwrap_or_return_handled_error!(user_service::authenticate_user_with_password(client, &user).await, Player);

    match unwrap_or_return_handled_error!(player_dao::get_player_with_id(conn, persisted_user.id).await, Player) {
        Some(found_player) => TypedHttpResponse::return_standard_response(200, found_player.clear_all_sensitive_fields()),
        None => TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Could not find player with id. Something went wrong.")),
    }
}