use std::sync::Arc;

use actix_web::{post, get, web::{self, Data, Json, Path}};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;
use league_types::{dto::league::LeagueForCreationDto, domain::league::League};

use crate::service::league;


#[post("")]
pub async fn create_league(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, league: Json<LeagueForCreationDto>) -> TypedHttpResponse<League> {
    league::create_league(&conn, &client, league.0).await
}

#[get("/nearme/{page}")]
pub async fn get_open_leagues_in_my_area(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, user: Json<UserForAuthenticationDto>, page: Path<u16>) -> TypedHttpResponse<Vec<League>> { // frontend should hit another endpoint if the user isn't registered
    league::get_open_leagues_in_my_area(&conn, &client, user.0, *page).await
}

#[get("/country/{country}/{page}")]
pub async fn get_leagues_in_country(conn: Data<Arc<MySqlPool>>, path_args: Path<(String, u16)>) -> TypedHttpResponse<Vec<League>> { // frontend should hit another endpoint if the user isn't registered
    league::get_leagues_in_country(&conn, &path_args.0, path_args.1).await
}

#[get("/{league_id}")]
pub async fn get_specific_league(conn: Data<Arc<MySqlPool>>, league_id: web::Path<u32>) -> TypedHttpResponse<League> {
    league::get_league(&conn, *league_id).await
}

#[post("/player/{player_id}/{page}")]
pub async fn get_leagues_hosted_by_player(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, user: Json<UserForAuthenticationDto>, path_args: web::Path<(u32, u16)>) -> TypedHttpResponse<Vec<League>> {
    league::get_leagues_hosted_by_player(&conn, &client, user.0, path_args.0, path_args.1).await
}

#[get("/place/{place_id}/{page}")]
pub async fn get_leagues_in_place(conn: Data<Arc<MySqlPool>>, path_args: web::Path<(u32, u16)>) -> TypedHttpResponse<Vec<League>> {
    league::get_leagues_in_place(&conn, path_args.0, path_args.1).await
}