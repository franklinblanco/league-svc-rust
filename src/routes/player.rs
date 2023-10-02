use std::sync::Arc;

use actix_web::{
    get, post, put,
    web::{self, Json, Path},
    HttpRequest,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::{
    domain::player::Player,
    dto::{
        player::*,
        player_metadata::{PlayerIds, PlayerMetadata},
    },
};
use reqwest::Client;
use sqlx::PgPool;
use user_lib::domain::token::Token;

use crate::{service::player, authenticate};

#[post("")]
pub async fn create_player_profile(
    conn: web::Data<Arc<PgPool>>,
    client: web::Data<Arc<Client>>,
    player: Json<PlayerForCreationDto>,
) -> TypedResponse<Token> {
    player::create_player_profile(&conn, &client, player.0).await.to_response()
}

#[put("")]
pub async fn edit_player_profile(
    conn: web::Data<Arc<PgPool>>,
    client: web::Data<Arc<Client>>,
    request: HttpRequest,
    player: Json<PlayerForUpdateDto>,
) -> TypedResponse<Player> {
    let user_id = authenticate!(request, &conn);
    player::edit_player_profile(&conn, &client, player.0).await.to_response()
}

#[post("/login")]
pub async fn login(
    conn: web::Data<Arc<PgPool>>,
    client: web::Data<Arc<Client>>,
    user: Json<PlayerForLoginDto>,
) -> TypedResponse<Token> {
    player::login(&conn, &client, user.0).await.to_response()
}

#[get("/profile/{player_id}")]
pub async fn get_player_profile(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<PlayerProfileDto> {
    let user_id = authenticate!(request, &conn);
    player::get_player_profile(&conn, *player_id).await.to_response()
}

#[get("/trusted/{player_id}")]
pub async fn get_player_trusted_list(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    player::get_player_trusted_list(&conn, *player_id).await.to_response()
}

#[get("/trusted_by/{player_id}")]
pub async fn get_player_trusted_by_list(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    player::get_player_trusted_by_list(&conn, *player_id).await.to_response()
}
//TODO: Verify phone number (prefferably in user-svc)
//TODO: Verify ID

/// Method to be called to get a large amount of player info such as name, profile picture url, etc...
#[post("/bulk")]
pub async fn get_player_metadata_bulk(
    conn: web::Data<Arc<PgPool>>,
    client: web::Data<Arc<Client>>,
    ids: web::Json<PlayerIds>,
    request: HttpRequest,
) -> TypedResponse<Vec<PlayerMetadata>> {
    let user_id = authenticate!(request, &conn);
    player::get_player_metadata_bulk(&conn, ids.0).await.to_response()
}
