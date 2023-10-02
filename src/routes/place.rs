use std::sync::Arc;

use actix_web::{
    get, post,
    web::{Data, Json, Path}, HttpRequest,
};
use actix_web_utils::extensions::typed_response::TypedResponse;
use league_types::domain::place::Place;
use reqwest::Client;
use sqlx::PgPool;

use crate::{service::place, authenticate};

#[get("/country/{country}/page/{page}")]
pub async fn get_places_for_country_paged(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(String, i64)>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    place::get_places_for_country_paged(&conn, path_args.0.clone(), path_args.1).await.to_response()
}

#[get("/sport/{sport_id}/page/{page}")]
pub async fn get_places_for_sport(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(i32, i64)>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    place::get_places_for_sport(&conn, path_args.0, path_args.1).await.to_response()
}

#[post("/nearme/{page}")]
pub async fn get_places_near_me(
    conn: Data<Arc<PgPool>>,
    client: Data<Arc<Client>>,
    request: HttpRequest,
    page: Path<i64>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    place::get_places_near_me(&conn, &client, user.0, *page).await.to_response()
}
