use std::sync::Arc;

use actix_web::{
    get, post,
    web::{self, Data, Json, Path}, HttpRequest,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::{domain::league::League, dto::league::LeagueForCreationDto};
use sqlx::PgPool;

use crate::{service::league, authenticate, finish_tx, create_tx};

#[post("")]
pub async fn create_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    league: Json<LeagueForCreationDto>,
) -> TypedResponse<League> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league::create_league(&mut *transaction, league.0, user_id).await;
    finish_tx!(response, transaction)
}

#[post("/nearme/{page}")]
pub async fn get_open_leagues_in_my_area(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    page: Path<i64>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    // frontend should hit another endpoint if the user isn't registered
    let mut transaction = create_tx!(conn);
    let response = league::get_open_leagues_in_my_area(&mut *transaction, *page, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/country/{country}/{page}")]
pub async fn get_leagues_in_country(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(String, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    // frontend should hit another endpoint if the user isn't registered
    let mut transaction = create_tx!(conn);
    let response = league::get_leagues_in_country(&mut *transaction, &path_args.0, path_args.1, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/{league_id}")]
pub async fn get_specific_league(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    league_id: web::Path<i32>,
) -> TypedResponse<League> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league::get_league(&mut *transaction, *league_id, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/player/{user_id}/{page}")]
pub async fn get_leagues_hosted_by_player(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: web::Path<(i32, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league::get_leagues_hosted_by_player(&mut *transaction, path_args.0, path_args.1, user_id).await;
    finish_tx!(response, transaction)
}

#[get("/place/{place_id}/{page}")]
pub async fn get_leagues_in_place(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: web::Path<(i32, i64)>,
) -> TypedResponse<Vec<League>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league::get_leagues_in_place(&mut *transaction, path_args.0, path_args.1, user_id).await;
    finish_tx!(response, transaction)
}

#[post("/league/{league_id}/age")]
pub async fn get_average_league_age(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    league_id: web::Path<i32>,
) -> TypedResponse<u8> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = league::get_average_league_age(&mut *transaction, *league_id, user_id).await;
    finish_tx!(response, transaction)
}

//TODO: Leave league