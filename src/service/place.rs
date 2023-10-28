use actix_web_utils::{ServiceResponse, x_u_res_db_or_sr, service_error};
use err::ServiceError as SE;
use league_types::domain::place::Place;
use sqlx::PgConnection;

use crate::dao::{place_dao, player_dao::get_player_with_id};

pub async fn get_places_for_country_paged(
    conn: &mut PgConnection,
    country: String,
    page: i64,
    _user_id: i32,
) -> ServiceResponse<Vec<Place>> {
    let res = x_u_res_db_or_sr!(
        place_dao::get_places_with_country_paged(conn, country, page).await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    
    service_error!(404, SE::NotFoundError { message: "No places found for your country.".into()})
}

pub async fn get_places_for_sport(
    conn: &mut PgConnection,
    sport_id: i32,
    page: i64,
    _user_id: i32,
) -> ServiceResponse<Vec<Place>> {
    let res = x_u_res_db_or_sr!(
        place_dao::get_place_with_sport_id_paged(conn, sport_id, page)
            .await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    service_error!(404, SE::NotFoundError { message: "No places found for selected sport.".into()})
}

pub async fn get_places_near_me(
    conn: &mut PgConnection,
    page: i64,
    user_id: i32,
) -> ServiceResponse<Vec<Place>> {
    let player = match x_u_res_db_or_sr!(
        get_player_with_id(conn, user_id).await
    ) {
        Some(player) => player,
        None => {
            return service_error!(404, SE::NotFoundError { message: "Player profile not found".into()})
        }
    };


    let res = x_u_res_db_or_sr!(
        place_dao::get_places_with_country_paged(
            conn,
            player.country,
            page,
        )
        .await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    service_error!(404, SE::NotFoundError { message: "No places found for your country.".into()})
}

pub async fn insert_all_places_from_list(conn: &mut PgConnection, ) {
    let all_places_persisted = match place_dao::get_all_places(conn).await {
        Ok(places) => places,
        Err(e) => panic!("{}", e.to_string()),
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
            Err(e) => panic!("{}", e.to_string()),
        }
    }
}
