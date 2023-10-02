use std::sync::Arc;

use actix_web::{
    post, put,
    web::{Data, Json, Path}, HttpRequest,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::{
    domain::{
        enums::league_player_status::ApprovalStatus, league::League, league_player::LeaguePlayer,
        player::Player,
    },
    dto::league_player::JoinRequest,
};
use reqwest::Client;
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
    client: Data<Arc<Client>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league_player::get_league_request_status(&mut *transaction, &client, join_req.0, user_id).await;
    finish_tx!(response, transaction)
}
#[put("/request/{status}")]
pub async fn change_league_request_status(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    new_status: Path<ApprovalStatus>,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    league_player::change_league_request_status(&conn, &client, new_status.to_owned(), join_req.0)
        .await.to_response()
}
#[post("/leagues/{page}")]
pub async fn get_all_leagues_player_has_applied_to(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
    page: Path<i64>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    league_player::get_all_leagues_player_has_applied_to(&conn, &client, join_req.0, *page).await.to_response()
}
#[post("/players")]
pub async fn get_all_players_in_league(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<Vec<Player>> {
    let user_id = authenticate!(request, &conn);
    league_player::get_all_players_in_league(&conn, &client, join_req.0).await.to_response()
}
#[post("/leave")]
pub async fn leave_league(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    join_req: Json<JoinRequest>,
) -> TypedResponse<LeaguePlayer> {
    let user_id = authenticate!(request, &conn);
    league_player::leave_league(&conn, &client, join_req.0).await.to_response()
}
