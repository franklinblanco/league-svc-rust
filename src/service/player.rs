use actix_web_utils::{extensions::typed_response::TypedResponse, u_opt_or_sr, ServiceResponse, u_res_or_sr};
use err::{MessageResource, x_u_res_db_or_res};
use league_types::{
    domain::player::Player,
    dto::{
        player::{PlayerForCreationDto, PlayerForUpdateDto, PlayerProfileDto},
        player_metadata::{PlayerIds, PlayerMetadata},
    },
};
use reqwest::Client;
use sqlx::{PgPool, PgConnection};
use user_lib::domain::credential::CredentialType;
use user_lib::domain::token::Token;
use user_lib::dto::credential::CredentialDto;
use user_lib::dto::users::UserRegisterPayload;
use user_lib::service::user::register_user;

use crate::{
    dao::{player_dao, trust_dao},
    util::converter,
};

pub async fn create_player_profile(
    conn: &mut PgConnection,
    client: &Client,
    player: PlayerForCreationDto,
) -> ServiceResponse<Token> {
    let mut connection = conn.acquire().await.expect("Error acquiring connection.");
    
    let register_user_payload = UserRegisterPayload {
        credentials: Vec::from([CredentialDto { credential: player.phone_number, credential_type: CredentialType::PhoneNumber }]),
        password: player.password,
        name: player.name,
    };
    let persisted_token =
        u_res_or_sr!(register_user(&mut connection, register_user_payload).await);
    let mut player_to_persist = Player::from(player);
    player_to_persist.id = persisted_token.user_id as i32;
    
    x_u_res_db_or_res!(player_dao::insert_player(conn, player_to_persist).await);
    TypedResponse::return_standard_response(200, persisted_token)
}
//TODO: Sign in & forgot password?

/// Called to update any detail in the player profile
pub async fn edit_player_profile(
    conn: &mut PgConnection,
    client: &Client,
    player: PlayerForUpdateDto,
) -> ServiceResponse<Player> {
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
        player_dao::get_player_with_id(conn, persisted_user.id as i32).await,
        Player
    ) {
        Some(found_player) => found_player,
        None => {
            return TypedResponse::return_standard_error(
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
    let updated_player = match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, persisted_user.id as i32).await,
        Player
    ) {
        Some(found_player) => found_player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };
    //  Debating in between an empty response with an OK or a more elaborate response with the updated Player.
    return TypedResponse::return_standard_response(200, updated_player);
}
//TODO: Verify user phone number
pub async fn login(
    conn: &mut PgConnection,
    client: &Client,
    mut user: UserForLoginDto,
) -> ServiceResponse<Token> {
    user.app = APP_NAME.to_string();
    let persisted_token = unwrap_or_return_handled_error!(
        user_service::authenticate_user_with_password(client, &user).await,
        Token
    );

    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, persisted_token.user_id as i32).await,
        Token
    ) {
        Some(_) => TypedResponse::return_standard_response(200, persisted_token),
        None => TypedResponse::return_standard_error(
            404,
            MessageResource::new_from_str("Could not find player with id. Something went wrong."),
        ),
    }
}

pub async fn get_player_profile(
    conn: &mut PgConnection,
    player_id: i32,
) -> ServiceResponse<PlayerProfileDto> {
    let persisted_player = match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        PlayerProfileDto
    ) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
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

    TypedResponse::return_standard_response(
        200,
        PlayerProfileDto::new_from_player_and_counts(
            &persisted_player,
            trusted_player_count.count,
            trusted_by_player_count.count,
        ),
    )
}

pub async fn get_player_trusted_list(
    conn: &mut PgConnection,
    player_id: i32,
) -> ServiceResponse<Vec<Player>> {
    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        Vec<Player>
    ) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };

    match player_dao::get_all_trusted_players(conn, player_id).await {
        Ok(players) => TypedResponse::return_standard_response(
            200,
            players
                .into_iter()
                .map(|player| Player::clear_all_sensitive_fields(player))
                .collect(),
        ),
        Err(e) => TypedResponse::return_standard_error(500, MessageResource::from(e.error)),
    }
}

pub async fn get_player_trusted_by_list(
    conn: &mut PgConnection,
    player_id: i32,
) -> ServiceResponse<Vec<Player>> {
    match unwrap_or_return_handled_error!(
        player_dao::get_player_with_id(conn, player_id).await,
        Vec<Player>
    ) {
        Some(player) => player,
        None => {
            return TypedResponse::return_standard_error(
                404,
                MessageResource::new_from_str(
                    "Could not find player with id. Something went wrong.",
                ),
            )
        }
    };

    match player_dao::get_all_players_that_trust_player(conn, player_id).await {
        Ok(players) => TypedResponse::return_standard_response(
            200,
            players
                .into_iter()
                .map(|player| Player::clear_all_sensitive_fields(player))
                .collect(),
        ),
        Err(e) => TypedResponse::return_standard_error(500, MessageResource::from(e.error)),
    }
}

pub async fn get_player_metadata_bulk(
    conn: &mut PgConnection,
    player_ids: PlayerIds,
) -> ServiceResponse<Vec<PlayerMetadata>> {
    let player_metadata_list = unwrap_or_return_handled_error!(
        player_dao::get_players_bulk(conn, player_ids.ids).await,
        Vec<PlayerMetadata>
    );
    TypedResponse::return_standard_response(200, player_metadata_list)
}
