use std::sync::Arc;

use actix_web::{get, post, web::{Path, Json, Data}};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;
use league_types::{domain::{place::Place}};

use crate::service::place;

#[get("/country/{country}/page/{page}")]
pub async fn get_places_for_country_paged(conn: Data<Arc<MySqlPool>>, path_args: Path<(String, u16)> ) -> TypedHttpResponse<Vec<Place>> {
    place::get_places_for_country_paged(&conn, path_args.0.clone(), path_args.1).await
}

#[get("/sport/{sport_id}/page/{page}")]
pub async fn get_places_for_sport(conn: Data<Arc<MySqlPool>>, path_args: Path<(u32, u16)>) -> TypedHttpResponse<Vec<Place>> {
    place::get_places_for_sport(&conn, path_args.0, path_args.1).await
}

#[post("/nearme/{page}")]
pub async fn get_places_near_me(conn: Data<Arc<MySqlPool>>, client: Data<Arc<Client>>, user: Json<UserForAuthenticationDto>, page: Path<u16>) -> TypedHttpResponse<Vec<Place>> {
    place::get_places_near_me(&conn, &client, user.0, *page).await
}
