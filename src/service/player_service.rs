use actix_web::http::StatusCode;
use actix_web_utils::{extensions::typed_response::TypedHttpResponse, dtos::message::MessageResource};
use dev_communicators::middleware::{user_svc::user_service::create_user};
use dev_dtos::{domain::user::{credential_type::CredentialType, token::Token}, dtos::user::user_dtos::UserForCreationDto, enums::error::Error};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player_dtos::PlayerForCreationDto, util::env_util::APP_NAME, dao::player_dao, domain::player::Player};


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
        Err(err) => match err {
            Error::UnexpectedStatusCode(_, actual, errorstr) => return TypedHttpResponse::return_standard_error(StatusCode::from_u16(actual).unwrap(), MessageResource::new_from_err(errorstr)),
            _ => return TypedHttpResponse::return_empty_response(StatusCode::INTERNAL_SERVER_ERROR)
        },
    };

    match player_dao::insert_player(conn, Player::new_from_creation_dto(&player, &persisted_token.user_id)).await {
        Ok(_) => TypedHttpResponse::return_standard_response(StatusCode::OK, persisted_token),
        Err(err) => TypedHttpResponse::return_standard_error(StatusCode::INTERNAL_SERVER_ERROR, MessageResource::new_from_err(err.to_string()))
    }
}