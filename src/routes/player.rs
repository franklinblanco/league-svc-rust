use std::sync::Arc;

use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::{domain::user::{token::Token}, dtos::user::user_dtos::UserForLoginDto};
use actix_web::{post, web::{Json, self, Path}, put, get};
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::player::{PlayerForCreationDto, PlayerForUpdateDto, PlayerProfileDto}, service::player, domain::player::Player};

#[post("")]
pub async fn create_player_profile(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, player: Json<PlayerForCreationDto>) -> TypedHttpResponse<Token> {
    player::create_player_profile(&db_conn, &client, player.0).await
}

#[put("")]
pub async fn edit_player_profile(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, player: Json<PlayerForUpdateDto>) -> TypedHttpResponse<Player> {
    player::edit_player_profile(&db_conn, &client, player.0).await
}

#[post("/login")]
pub async fn login(db_conn: web::Data<Arc<MySqlPool>>, client: web::Data<Arc<Client>>, user: Json<UserForLoginDto>) -> TypedHttpResponse<Token> {
    player::login(&db_conn, &client, user.0).await
}

#[get("/profile/{player_id}")]
pub async fn get_player_profile(db_conn: web::Data<Arc<MySqlPool>>, player_id: Path<i32>) -> TypedHttpResponse<PlayerProfileDto> {
    player::get_player_profile(&db_conn, *player_id).await
}

#[get("/trusted/{player_id}")]
pub async fn get_player_trusted_list(db_conn: web::Data<Arc<MySqlPool>>, player_id: Path<i32>) -> TypedHttpResponse<Vec<Player>> {
    player::get_player_trusted_list(&db_conn, *player_id).await
}

#[get("/trusted_by/{player_id}")]
pub async fn get_player_trusted_by_list(db_conn: web::Data<Arc<MySqlPool>>, player_id: Path<i32>) -> TypedHttpResponse<Vec<Player>> {
    player::get_player_trusted_by_list(&db_conn, *player_id).await
}
//TODO: Verify phone number (prefferably in user-svc)
//TODO: Verify ID
