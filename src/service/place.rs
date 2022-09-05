use actix_web_utils::{extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error, dtos::message::MessageResource};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{domain::place::Place, dao::{player_dao::get_player_with_id, place_dao}, util::env_util::APP_NAME};


pub async fn get_places_for_country_paged(conn: &MySqlPool, country: String, page: i32) -> TypedHttpResponse<Vec<Place>> {
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;

    let res = unwrap_or_return_handled_error!(place_dao::get_places_with_country_paged(conn, country, from_row, to_row).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))
}

pub async fn get_places_for_sport(conn: &MySqlPool, sport_id: i32, page: i32) -> TypedHttpResponse<Vec<Place>> {
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;

    let res = unwrap_or_return_handled_error!(place_dao::get_place_with_sport_id_paged(conn, sport_id, from_row, to_row).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))

}

pub async fn get_places_near_me(conn: &MySqlPool, client: &Client, mut user_for_auth: UserForAuthenticationDto, page: i32) -> TypedHttpResponse<Vec<Place>> {
    user_for_auth.app = APP_NAME.to_string();
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<Place>
    );
    let player = match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id).await, Vec<Place>) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Player profile not found.")),
    };
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;

    let res = unwrap_or_return_handled_error!(place_dao::get_places_with_country_paged(conn, player.country, from_row, to_row).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))

}
