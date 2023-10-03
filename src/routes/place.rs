use std::sync::Arc;

use actix_web::{
    get, post,
    web::{Data, Path}, HttpRequest,
};
use actix_web_utils::extensions::{typed_response::TypedResponse, service_response::IntoResponse};
use league_types::domain::place::Place;
use sqlx::PgPool;

use crate::{service::place, authenticate, finish_tx, create_tx};

#[get("/country/{country}/page/{page}")]
pub async fn get_places_for_country_paged(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(String, i64)>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = place::get_places_for_country_paged(&mut *transaction, path_args.0.clone(), path_args.1, user_id).await;
    finish_tx!(response, transaction)

}

#[get("/sport/{sport_id}/page/{page}")]
pub async fn get_places_for_sport(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    path_args: Path<(i32, i64)>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = place::get_places_for_sport(&mut *transaction, path_args.0, path_args.1, user_id).await;
    finish_tx!(response, transaction)
}

#[post("/nearme/{page}")]
pub async fn get_places_near_me(
    conn: Data<Arc<PgPool>>,
    request: HttpRequest,
    page: Path<i64>,
) -> TypedResponse<Vec<Place>> {
    let user_id = authenticate!(request, &conn);
    let mut transaction = create_tx!(conn);
    let response = place::get_places_near_me(&mut *transaction, *page, user_id).await;
    finish_tx!(response, transaction)
}
