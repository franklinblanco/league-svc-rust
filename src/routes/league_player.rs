use std::sync::Arc;

use actix_web::{
    post, put,
    web::{Data, Json, Path}, HttpRequest, get,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::{
    domain::{
        enums::league_player_status::ApprovalStatus, league::League, league_player::LeaguePlayer,
        player::Player,
    },
    dto::league_player::JoinRequest,
};
use sqlx::PgPool;

use crate::{service::league_player, authenticate, create_tx, finish_tx};

#[post("/request")]
pub async fn request_to_join_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::request_to_join_league(&mut *transaction, join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
#[post("/request/status")]
pub async fn get_league_request_status(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::get_league_request_status(&mut *transaction, join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
#[put("/request/{status}")]
pub async fn change_league_request_status(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    new_status: Path<ApprovalStatus>,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::change_league_request_status(&mut *transaction, new_status.to_owned(), join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
#[get("/leagues/{page}")]
pub async fn get_all_leagues_player_has_applied_to(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    page: Path<i64>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::get_all_leagues_player_has_applied_to(&mut *transaction, *page, user_id).await;
    finish_tx!(response, transaction)
}
#[post("/players")]
pub async fn get_all_players_in_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::get_all_players_in_league(&mut *transaction, join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
#[post("/leave")]
pub async fn leave_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::leave_league(&mut *transaction, join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
