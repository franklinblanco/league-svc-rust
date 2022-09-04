use actix_web::{get, post, web::{Path, Json}};
use actix_web_utils::extensions::typed_response::TypedHttpResponse;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;

use crate::domain::place::Place;


#[get("/country/{country}")]
pub async fn get_places_for_country_paged(_country: Path<String>, ) -> TypedHttpResponse<Vec<Place>> {
    todo!()
}

#[get("/sport/{sport_id}")]
pub async fn get_places_for_sport(_sport_id: Path<i32>) -> TypedHttpResponse<Vec<Place>> {
    todo!()
}

#[post("/nearme")]
pub async fn get_places_near_me(_user: Json<UserForAuthenticationDto>) -> TypedHttpResponse<Vec<Place>> {
    todo!()
}
