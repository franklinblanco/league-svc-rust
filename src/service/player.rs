use actix_web_utils::{
    extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error,
};
use dev_communicators::middleware::user_svc::user_service::{self, create_user};
use dev_dtos::{
    domain::user::token::Token,
    dtos::user::user_dtos::{UserForAuthenticationDto, UserForLoginDto},
};
use err::MessageResource;
use league_types::{
    domain::player::Player,
    dto::{
        player::{PlayerForCreationDto, PlayerForUpdateDto, PlayerProfileDto},
        player_metadata::{PlayerIds, PlayerMetadata},
    },
    APP_NAME,
};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{
    dao::{player_dao, trust_dao},
    util::converter,
};

/// Self explanatory name
pub async fn create_player_profile(
    conn: &MySqlPool,
    client: &Client,
    player: PlayerForCreationDto,
) -> TypedHttpResponse<Token> {
    let user_for_creation = PlayerForCreationDto::convert_player_into_user_for_creation(&player);

    let persisted_token =
        unwrap_or_return_handled_error!(create_user(client, &user_for_creation).await, Token);
    let mut player_to_persist = Player::from(player);
    player_to_persist.id = persisted_token.user_id as u32;
    unwrap_or_return_handled_error!(
        500,
        player_dao::insert_player(conn, player_to_persist).await,
        Token
    );
    TypedHttpResponse::return_standard_response(200, persisted_token)
}
//TODO: Sign in & forgot password?

/// Called to update any detail in the player profile
pub async fn edit_player_profile(
    conn: &MySqlPool,
    client: &Client,
    player: PlayerForUpdateDto,
) -> TypedHttpResponse<Player> {
    let persisted_user = unwrap_or_return_handled_error!(
        user_service::authenticate_user_with_token(
            client,
            &UserForAuthenticationDto {
                app: APP_NAME.to_string(),
                id: player.user_id.to_string(),
                token: player.auth_token.clone()
            }
        )
        .await,
        Player
    );
    //  Attempt to find player in database with the user id that user service gave back
    let persisted_player = match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, persisted_user.id as u32).await,
        Player
    ) {
        Some(found_player) => found_player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };
    let player_to_update = unwrap_or_return_handled_error!(
        400,
        converter::update_player_struct(player, persisted_player),
        Player
    );
    unwrap_or_return_handled_error!(
        player_dao::update_player_with_id(conn, player_to_update).await,
        Player
    );
    //  Debating in between an empty response with an OK or a more elaborate response with the updated Player.
    return TypedHttpResponse::return_empty_response(200);
}
//TODO: Verify user phone number
pub async fn login(
    conn: &MySqlPool,
    client: &Client,
    mut user: UserForLoginDto,
) -> TypedHttpResponse<Token> {
    user.app = APP_NAME.to_string();
    let persisted_token = unwrap_or_return_handled_error!(
        user_service::authenticate_user_with_password(client, &user).await,
        Token
    );

    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, persisted_token.user_id as u32).await,
        Token
    ) {
        Some(_) => TypedHttpResponse::return_standard_response(200, persisted_token),
        None => TypedHttpResponse::return_standard_error(
            404,
            MessageResource::new_from_str("Could not find player with id. Something went wrong."),
        ),
    }
}

pub async fn get_player_profile(
    conn: &MySqlPool,
    player_id: u32,
) -> TypedHttpResponse<PlayerProfileDto> {
    let persisted_player = match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        PlayerProfileDto
    ) {
        Some(player) => player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };

    let trusted_player_count = unwrap_or_return_handled_error!(
        trust_dao::get_trusts_by_truster_id(conn, player_id).await,
        PlayerProfileDto
    );
    let trusted_by_player_count = unwrap_or_return_handled_error!(
        trust_dao::get_trusts_by_trustee_id(conn, player_id).await,
        PlayerProfileDto
    );

    TypedHttpResponse::return_standard_response(
        200,
        PlayerProfileDto::new_from_player_and_counts(
            &persisted_player,
            trusted_player_count.count,
            trusted_by_player_count.count,
        ),
    )
}

pub async fn get_player_trusted_list(
    conn: &MySqlPool,
    player_id: u32,
) -> TypedHttpResponse<Vec<Player>> {
    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        Vec<Player>
    ) {
        Some(player) => player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };

    match player_dao::get_all_trusted_players(conn, player_id).await {
        Ok(players) => TypedHttpResponse::return_standard_response(
            200,
            players
                .into_iter()
                .map(|player| Player::clear_all_sensitive_fields(player))
                .collect(),
        ),
        Err(e) => TypedHttpResponse::return_standard_error(500, MessageResource::from(e.error)),
    }
}

pub async fn get_player_trusted_by_list(
    conn: &MySqlPool,
    player_id: u32,
) -> TypedHttpResponse<Vec<Player>> {
    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        Vec<Player>
    ) {
        Some(player) => player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };

    match player_dao::get_all_players_that_trust_player(conn, player_id).await {
        Ok(players) => TypedHttpResponse::return_standard_response(
            200,
            players
                .into_iter()
                .map(|player| Player::clear_all_sensitive_fields(player))
                .collect(),
        ),
        Err(e) => TypedHttpResponse::return_standard_error(500, MessageResource::from(e.error)),
    }
}

pub async fn get_player_metadata_bulk(
    conn: &MySqlPool,
    player_ids: PlayerIds,
) -> TypedHttpResponse<Vec<PlayerMetadata>> {
    let player_metadata_list = unwrap_or_return_handled_error!(
        player_dao::get_players_bulk(conn, player_ids.ids).await,
        Vec<PlayerMetadata>
    );
    TypedHttpResponse::return_standard_response(200, player_metadata_list)
}
