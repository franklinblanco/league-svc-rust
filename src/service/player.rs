use actix_web_utils::{ServiceResponse, u_res_or_sr, x_u_res_db_or_sr, service_error, x_u_res_or_sr};
use err::ServiceError as SE;
use league_types::{
    domain::player::Player,
    dto::{
        player::{PlayerForCreationDto, PlayerForUpdateDto, PlayerProfileDto, PlayerForLoginDto},
        player_metadata::{PlayerIds, PlayerMetadata},
    },
};

use sqlx::PgConnection;
use user_lib::{domain::credential::CredentialType, dto::users::UserLoginPayload, service::user::password_login};
use user_lib::domain::token::Token;
use user_lib::dto::credential::CredentialDto;
use user_lib::dto::users::UserRegisterPayload;
use user_lib::service::user::register_user;

use crate::{
    dao::{player_dao, trust_dao},
    util::converter,
};

pub async fn create_player_profile(
    transaction: &mut PgConnection,
    player: PlayerForCreationDto,
) -> ServiceResponse<Token> {
    let cloned_player = player.clone();
    let register_user_payload = UserRegisterPayload {
        credentials: Vec::from([CredentialDto { credential: player.phone_number, credential_type: CredentialType::PhoneNumber }]),
        password: player.password,
        name: player.name,
    };
    let persisted_token =
        u_res_or_sr!(register_user(transaction, register_user_payload).await);
    let mut player_to_persist = Player::from(cloned_player);
    player_to_persist.id = persisted_token.user_id as i32;
    
    x_u_res_db_or_sr!(player_dao::insert_player(transaction, player_to_persist).await);
    Ok(persisted_token)
}
//TODO: Sign in & forgot password?

/// Called to update any detail in the player profile
pub async fn edit_player_profile(
    transaction: &mut PgConnection,
    player: PlayerForUpdateDto,
    user_id: i32,
) -> ServiceResponse<Player> {
    //  Attempt to find player in database with the user id that user service gave back
    let persisted_player = match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, user_id).await
    ) {
        Some(found_player) => found_player,
        None => {
            return service_error!(404, SE::NotFoundError { message: "Could not find player with id. Something went wrong.".into()})
        }
    };
    let player_to_update = x_u_res_or_sr!(
        converter::update_player_struct(player, persisted_player),
        SE::NotAllowed { message: String::new()}
    );
    Ok(x_u_res_db_or_sr!(player_dao::update_player_with_id(transaction, player_to_update).await))
}

pub async fn login(
    transaction: &mut PgConnection,
    user: PlayerForLoginDto,
) -> ServiceResponse<Token> {
    let persisted_token = u_res_or_sr!(
        password_login(transaction, UserLoginPayload {
            credential: user.phone_number,
            credential_type: CredentialType::PhoneNumber,
            password: user.password }).await
    );

    match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, persisted_token.user_id).await
    ) {
        Some(_) => Ok(persisted_token),
        None => service_error!(404, SE::NotFoundError { message: "Could not find player with id. Something went wrong.".into()}),
    }
}

pub async fn get_player_profile(
    transaction: &mut PgConnection,
    player_id: i32,
    _user_id: i32,
) -> ServiceResponse<PlayerProfileDto> {
    let persisted_player = match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, player_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError { message: "Could not find player with id. Something went wrong.".into()})
        }
    };

    let trusted_player_count = x_u_res_db_or_sr!(
        trust_dao::get_trusts_by_truster_id(transaction, player_id).await
    );
    let trusted_by_player_count = x_u_res_db_or_sr!(
        trust_dao::get_trusts_by_trustee_id(transaction, player_id).await
    );

    Ok(
        PlayerProfileDto::new_from_player_and_counts(
            &persisted_player,
            trusted_player_count.count,
            trusted_by_player_count.count,
        ),
    )
}

pub async fn get_player_trusted_list(
    transaction: &mut PgConnection,
    player_id: i32,
    _user_id: i32,
) -> ServiceResponse<Vec<Player>> {
    match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, player_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError { message: "Could not find player with id. Something went wrong.".into()})
        }
    };

    Ok(x_u_res_db_or_sr!(player_dao::get_all_trusted_players(transaction, player_id).await).into_iter()
    .map(|player| Player::clear_all_sensitive_fields(player))
    .collect())
}

pub async fn get_player_trusted_by_list(
    transaction: &mut PgConnection,
    player_id: i32,
    _user_id: i32,
) -> ServiceResponse<Vec<Player>> {
    match x_u_res_db_or_sr!(
        player_dao::get_player_with_id(transaction, player_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError { message: "Could not find player with id. Something went wrong.".into()})
        }
    };

    Ok(x_u_res_db_or_sr!(player_dao::get_all_players_that_trust_player(transaction, player_id).await).into_iter()
    .map(|player| Player::clear_all_sensitive_fields(player))
    .collect())
}

pub async fn get_player_metadata_bulk(
    transaction: &mut PgConnection,
    player_ids: PlayerIds,
    _user_id: i32,
) -> ServiceResponse<Vec<PlayerMetadata>> {
    let player_metadata_list = x_u_res_db_or_sr!(
        player_dao::get_players_bulk(transaction, player_ids.ids).await
    );
    Ok(player_metadata_list.into_iter().map(|p|p.into()).collect())
}
