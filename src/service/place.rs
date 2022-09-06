use actix_web_utils::{extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error, dtos::message::MessageResource};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{domain::place::Place, dao::{player_dao::get_player_with_id, place_dao}, util::{env_util::APP_NAME, repeat_utils::get_from_and_to_from_page}};


pub async fn get_places_for_country_paged(conn: &MySqlPool, country: String, page: u16) -> TypedHttpResponse<Vec<Place>> {
    let page_limits = match get_from_and_to_from_page(page) {
        Ok(res) => res,
        Err(message) => return TypedHttpResponse::return_standard_error(400, message),
    };

    let res = unwrap_or_return_handled_error!(place_dao::get_places_with_country_paged(conn, country, page_limits.0, page_limits.1).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))
}

pub async fn get_places_for_sport(conn: &MySqlPool, sport_id: u32, page: u16) -> TypedHttpResponse<Vec<Place>> {
    let page_limits = match get_from_and_to_from_page(page) {
        Ok(res) => res,
        Err(message) => return TypedHttpResponse::return_standard_error(400, message),
    };

    let res = unwrap_or_return_handled_error!(place_dao::get_place_with_sport_id_paged(conn, sport_id, page_limits.0, page_limits.1).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))

}

pub async fn get_places_near_me(conn: &MySqlPool, client: &Client, mut user_for_auth: UserForAuthenticationDto, page: u16) -> TypedHttpResponse<Vec<Place>> {
    user_for_auth.app = APP_NAME.to_string();
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<Place>
    );
    let player = match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as u32).await, Vec<Place>) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Player profile not found.")),
    };
    let page_limits = match get_from_and_to_from_page(page) {
        Ok(res) => res,
        Err(message) => return TypedHttpResponse::return_standard_error(400, message),
    };

    let res = unwrap_or_return_handled_error!(place_dao::get_places_with_country_paged(conn, player.country, page_limits.0, page_limits.1).await, Vec<Place>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No places found for your country."))

}


pub async fn insert_all_places_from_list(conn: &MySqlPool) {
    let all_places_persisted = match place_dao::get_all_places(conn).await {
        Ok(places) => places,
        Err(e) => panic!("{}", e.error.to_string()),
    };
    let all_places: Vec<Place> = match serde_json::from_str(include_str!("../../places.json")) {
        Ok(res) => match res {serde_json::Value::Array(arr) => arr.into_iter().map(|val|
            {
                let mut place = Place::new();
                place.name = val.get("name").unwrap().as_str().unwrap().to_string();
                place.sport_id = val.get("sport_id").unwrap().as_i64().unwrap() as u32;
                place.country = val.get("country").unwrap().as_str().unwrap().to_string();
                place.state = Some(val.get("state").unwrap().as_str().unwrap().to_string());
                place.city = val.get("city").unwrap().as_str().unwrap().to_string();
                place.address = val.get("address").unwrap().as_str().unwrap().to_string();
                place.maps_url = Some(val.get("maps_url").unwrap().as_str().unwrap().to_string());
                place.contact_number = Some(val.get("contact_number").unwrap().as_str().unwrap().to_string());
                place.picture_url = None;
                place
            }
        ).collect(),
        _ => panic!("No places found in places.json. Is this missing or what?"),
    },
        Err(e) => panic!("{}", e.to_string()),
    };
    if all_places_persisted.len() == all_places.len() {
        return;
    }
    for place in all_places {
        match place_dao::insert_place(conn, place).await {
            Ok(_) => {},
            Err(e) => panic!("{}", e.error.to_string())
        }
    }
}