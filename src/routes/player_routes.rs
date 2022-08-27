use std::sync::Arc;

use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::{domain::user::{token::Token}};
use actix_web::{post, web::{Json, self}};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player_dtos::PlayerForCreationDto, service::player_service};

#[post("/player")]
pub async fn create_player_profile(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, player: Json<PlayerForCreationDto>) -> TypedHttpResponse<Token> {
    player_service::create_player_profile(&db_conn, &client, player.0).await
}

