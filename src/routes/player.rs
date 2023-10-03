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
use sqlx::PgPool;
use user_lib::domain::token::Token;

use crate::{service::player, authenticate, create_tx, finish_tx};

#[post("")]
pub async fn create_player_profile(
    conn: web::Data<Arc<PgPool>>,
    player: Json<PlayerForCreationDto>,
) -> TypedResponse<Token> {
    let mut transaction = create_tx!(conn);
    let response = player::create_player_profile(&mut *transaction,  player.0).await;
    finish_tx!(response, transaction)
}

#[put("")]
pub async fn edit_player_profile(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player: Json<PlayerForUpdateDto>,
) -> TypedResponse<Player> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = player::edit_player_profile(&mut *transaction, player.0, user_id).await;
    finish_tx!(response, transaction)
}

#[post("/login")]
pub async fn login(
    conn: web::Data<Arc<PgPool>>,
    user: Json<PlayerForLoginDto>,
) -> TypedResponse<Token> {
    let mut transaction = create_tx!(conn);
    let response = player::login(&mut *transaction, user.0).await;
    finish_tx!(response, transaction)
}

#[get("/profile/{player_id}")]
pub async fn get_player_profile(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<PlayerProfileDto> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = player::get_player_profile(&mut *transaction, *player_id, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/trusted/{player_id}")]
pub async fn get_player_trusted_list(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = player::get_player_trusted_list(&mut *transaction, *player_id, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/trusted_by/{player_id}")]
pub async fn get_player_trusted_by_list(
    conn: web::Data<Arc<PgPool>>,
    request: HttpRequest,
    player_id: Path<i32>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = player::get_player_trusted_by_list(&mut *transaction, *player_id, user_id).await;
    finish_tx!(response, transaction)
}
//TODO: Verify phone number (prefferably in user-svc)
//TODO: Verify ID

/// Method to be called to get a large amount of player info such as name, profile picture url, etc...
#[post("/bulk")]
pub async fn get_player_metadata_bulk(
    conn: web::Data<Arc<PgPool>>,
    ids: web::Json<PlayerIds>,
    request: HttpRequest,
) -> TypedResponse<Vec<PlayerMetadata>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = player::get_player_metadata_bulk(&mut *transaction, ids.0, user_id).await;
    finish_tx!(response, transaction)
}
