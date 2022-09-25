use std::{sync::Arc};

use actix_web::{web::{Data, Json, Path}, post, put};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use reqwest::Client;
use sqlx::MySqlPool;
use league_types::{domain::{league_player::LeaguePlayer, player::Player, league::League, enums::league_player_status::LeaguePlayerStatus}, dto::league_player::JoinRequest};

use crate::service::league_player;

#[post("/request")]
pub async fn request_to_join_league(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, join_req: Json<JoinRequest>) -> TypedHttpResponse<LeaguePlayer> {
    league_player::request_to_join_league(&conn, &client, join_req.0).await
}
#[post("/request/status")]
pub async fn get_league_request_status(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, join_req: Json<JoinRequest>) -> TypedHttpResponse<LeaguePlayer> {
    league_player::get_league_request_status(&conn, &client, join_req.0).await
}
#[put("/request/{status}")]
pub async fn change_league_request_status(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, new_status: Path<LeaguePlayerStatus>, join_req: Json<JoinRequest>) -> TypedHttpResponse<LeaguePlayer> {
    league_player::change_league_request_status(&conn, &client, new_status.to_owned(), join_req.0).await
}
#[post("/leagues/{page}")]
pub async fn get_all_leagues_player_has_applied_to(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, join_req: Json<JoinRequest>, page: Path<u16>) -> TypedHttpResponse<Vec<League>> {
    league_player::get_all_leagues_player_has_applied_to(&conn, &client, join_req.0, *page).await
}
#[post("/players")]
pub async fn get_all_players_in_league(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, join_req: Json<JoinRequest>) -> TypedHttpResponse<Vec<Player>> {
    league_player::get_all_players_in_league(&conn, &client, join_req.0).await
}
