use actix_web_utils::{
    extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error,
};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use err::MessageResource;
use league_types::{domain::place::Place, APP_NAME};
use reqwest::Client;
use sqlx::PgPool;

use crate::dao::{place_dao, player_dao::get_player_with_id};

pub async fn get_places_for_country_paged(
    conn: &PgPool,
    country: String,
    page: i64,
) -> TypedHttpResponse<Vec<Place>> {

    let res = unwrap_or_return_handled_error!(
        place_dao::get_places_with_country_paged(conn, country, page).await,
        Vec<Place>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No places found for your country."),
    )
}

pub async fn get_places_for_sport(
    conn: &PgPool,
    sport_id: i32,
    page: i64,
) -> TypedHttpResponse<Vec<Place>> {


    let res = unwrap_or_return_handled_error!(
        place_dao::get_place_with_sport_id_paged(conn, sport_id, page)
            .await,
        Vec<Place>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No places found for your country."),
    )
}

pub async fn get_places_near_me(
    conn: &PgPool,
    client: &Client,
    mut user_for_auth: UserForAuthenticationDto,
    page: i64,
) -> TypedHttpResponse<Vec<Place>> {
    user_for_auth.app = APP_NAME.to_string();
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<Place>
    );
    let player = match unwrap_or_return_handled_error!(
        get_player_with_id(conn, user.id as i32).await,
        Vec<Place>
    ) {
        Some(player) => player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Player profile not found."),
            )
        }
    };


    let res = unwrap_or_return_handled_error!(
        place_dao::get_places_with_country_paged(
            conn,
            player.country,
            page,
        )
        .await,
        Vec<Place>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No places found for your country."),
    )
}

pub async fn insert_all_places_from_list(conn: &PgPool) {
    let all_places_persisted = match place_dao::get_all_places(conn).await {
        Ok(places) => places,
        Err(e) => panic!("{}", e.error.to_string()),
    };
    let all_places: Vec<Place> = match serde_json::from_str(include_str!("../../places.json")) {
        Ok(res) => match res {
            serde_json::Value::Array(arr) => arr
                .into_iter()
                .map(|val| {
                    let mut place = Place::default();
                    place.name = val.get("name").unwrap().as_str().unwrap().to_string();
                    place.sport_id = val.get("sport_id").unwrap().as_i64().unwrap() as i32;
                    place.country = val.get("country").unwrap().as_str().unwrap().to_string();
                    place.state = Some(val.get("state").unwrap().as_str().unwrap().to_string());
                    place.city = val.get("city").unwrap().as_str().unwrap().to_string();
                    place.address = val.get("address").unwrap().as_str().unwrap().to_string();
                    place.maps_url =
                        Some(val.get("maps_url").unwrap().as_str().unwrap().to_string());
                    place.contact_number = Some(
                        val.get("contact_number")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    );
                    place.picture_url = None;
                    place
                })
                .collect(),
            _ => panic!("No places found in places.json. Is this missing or what?"),
        },
        Err(e) => panic!("{}", e.to_string()),
    };
    if all_places_persisted.len() == all_places.len() {
        return;
    }
    for place in all_places {
        match place_dao::insert_place(conn, place).await {
            Ok(_) => {}
            Err(e) => panic!("{}", e.error.to_string()),
        }
    }
}
