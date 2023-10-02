use std::sync::Arc;

use actix_web::{
    get, post,
    web::{self, Data, Json, Path}, HttpRequest,
};
use actix_web_utils::extensions::typed_response::TypedResponse;
use league_types::{domain::league::League, dto::league::LeagueForCreationDto};
use reqwest::Client;
use sqlx::PgPool;

use crate::{service::league, authenticate};

#[post("")]
pub async fn create_league(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    league: Json<LeagueForCreationDto>,
) -> TypedResponse<League> {
    let user_id = authenticate!(request, &conn);
    league::create_league(&conn, &client, league.0).await.to_response()
}

#[post("/nearme/{page}")]
pub async fn get_open_leagues_in_my_area(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    page: Path<i64>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    // frontend should hit another endpoint if the user isn't registered
    league::get_open_leagues_in_my_area(&conn, &client, user.0, *page).await.to_response()
}

#[get("/country/{country}/{page}")]
pub async fn get_leagues_in_country(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(String, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    // frontend should hit another endpoint if the user isn't registered
    league::get_leagues_in_country(&conn, &path_args.0, path_args.1).await.to_response()
}

#[get("/{league_id}")]
pub async fn get_specific_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    league_id: web::Path<i32>,
) -> TypedResponse<League> {
    let user_id = authenticate!(request, &conn);
    league::get_league(&conn, *league_id).await.to_response()
}

#[post("/player/{player_id}/{page}")]
pub async fn get_leagues_hosted_by_player(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    path_args: web::Path<(i32, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    league::get_leagues_hosted_by_player(&conn, &client, user.0, path_args.0, path_args.1).await.to_response()
}

#[get("/place/{place_id}/{page}")]
pub async fn get_leagues_in_place(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: web::Path<(i32, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    league::get_leagues_in_place(&conn, path_args.0, path_args.1).await.to_response()
}

#[post("/league/{league_id}/age")]
pub async fn get_average_league_age(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    league_id: web::Path<i32>,
) -> TypedResponse<u8> {
    let user_id = authenticate!(request, &conn);
    league::get_average_league_age(&conn, &client, user.0, *league_id).await.to_response()
}
