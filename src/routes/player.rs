use std::sync::Arc;

use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::{domain::user::{token::Token}, dtos::user::user_dtos::UserForLoginDto};
use actix_web::{post, web::{Json, self}, put};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player::{PlayerForCreationDto, PlayerForUpdateDto}, service::player, domain::player::Player};

#[post("")]
pub async fn create_player_profile(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, player: Json<PlayerForCreationDto>) -> TypedHttpResponse<Token> {
    player::create_player_profile(&db_conn, &client, player.0).await
}
#[put("")]
pub async fn edit_player_profile(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, player: Json<PlayerForUpdateDto>) -> TypedHttpResponse<Player> {
    player::edit_player_profile(&db_conn, &client, player.0).await
}
#[post("/login")]
pub async fn login(_db_conn: web::Data<Arc<MySqlPool>>, _client: web::Data<Arc<Client>>, _user: Json<UserForLoginDto>) -> TypedHttpResponse<Player> {
    todo!()
}
//TODO: Verify phone number (prefferably in user-svc)
//TODO: Verify ID
