use std::sync::Arc;

use actix_web::{HttpResponse, post, get, web::{self, Data, Json}};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{domain::league::League, service::league::{self}, dto::league::LeagueForCreationDto};

#[post("")]
pub async fn create_league(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, league: Json<LeagueForCreationDto>) -> TypedHttpResponse<League> {
    league::create_league(&conn, &client, league.0).await
}

#[get("/nearme")]
pub async fn get_open_leagues_in_my_area(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, user: Json<UserForAuthenticationDto>) -> TypedHttpResponse<Vec<League>> { // frontend should hit another endpoint if the user isn't registered
    league::get_open_leagues_in_my_area(&conn, &client, user.0).await
}

#[get("/{league_id}")]
pub async fn get_specific_league(conn: Data<Arc<MySqlPool>>, league_id: web::Path<i32>) -> TypedHttpResponse<League> {
    league::get_league(&conn, *league_id).await
}

#[get("/player/{player_id}")]
pub async fn get_leagues_hosted_by_player(_player_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/place/{place_id}")]
pub async fn get_leagues_in_place(_place_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().finish()
}