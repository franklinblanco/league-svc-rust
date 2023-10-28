use actix_web_utils::{
    ServiceResponse, x_u_res_db_or_sr, service_error,
};

use chrono::Utc;
use err::ServiceError as SE;
use league_types::{domain::{league::League, enums::league_player_status::LeaguePlayerStatus}, dto::league::LeagueForCreationDto};
use sqlx::PgConnection;

use crate::
    dao::{
        league_dao::*,
        player_dao::{self, *},
    };

/// Create a league.
pub async fn create_league(
    transaction: &mut PgConnection,
    league: LeagueForCreationDto,
    user_id: i32,
) -> ServiceResponse<League> {
    // TODO: Validation: League time must be in the future
    // TODO: Validate user doesn't have more than 10 leagues open?
    // TODO: ALL VALIDATIONS: League maxplayers > 1
    // TODO: PLACE ID EXISTS, COST TO JOIN ISN'T NEGATIVE, ETC
    Ok(x_u_res_db_or_sr!(insert_league(transaction, League::from_league_creation_dto(league, user_id)).await))
}

/// Used to get a specific league
pub async fn get_league(transaction: &mut PgConnection, id: i32, _user_id: i32) -> ServiceResponse<League> {
    match x_u_res_db_or_sr!(get_league_with_id(transaction, id).await) {
        Some(league) => Ok(league),
        None => service_error!(404, SE::NotFoundError { message: "League not found.".into()})
    }
}

/// This route infers the player's area by his country & city.
pub async fn get_open_leagues_in_my_area(
    transaction: &mut PgConnection,
    page: i64,
    user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let player = match x_u_res_db_or_sr!(
        get_player_with_id(transaction, user_id).await
    ) {
        Some(player) => player,
        None => return service_error!(404, SE::NotFoundError { message: "Player profile not found.".into()})
    };

    let res = x_u_res_db_or_sr!(
        get_leagues_by_country_limited_to(transaction, player.country, page).await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    service_error!(404, SE::NotFoundError { message: "No leagues found for your country.".into()})
}

/// This route is used to get leagues from a country
pub async fn get_leagues_in_country(
    transaction: &mut PgConnection,
    country: &String,
    page: i64,
    _user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let res = x_u_res_db_or_sr!(
        get_leagues_by_country_limited_to(transaction, country.clone(), page)
            .await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    service_error!(404, SE::NotFoundError { message: "No leagues found for your country.".into()})
}

/// This route is used to get leagues from a country
pub async fn get_leagues_in_place(
    transaction: &mut PgConnection,
    place_id: i32,
    page: i64,
    _user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let res = x_u_res_db_or_sr!(
        get_leagues_by_in_place_limited_to(transaction, place_id, page).await
    );
    if res.len() > 0 {
        return Ok(res);
    }
    service_error!(404, SE::NotFoundError { message: "No leagues found for place.".into()})
}

/// Only shows non unlisted leagues //TODO: Make a new endpoint to get MyLeagues (Only callable by the owner)
pub async fn get_leagues_hosted_by_player(
    transaction: &mut PgConnection,
    player_id: i32,
    page: i64,
    _user_id: i32,
) -> ServiceResponse<Vec<League>> {
    let leagues = x_u_res_db_or_sr!(
        get_leagues_by_player_limited_to(transaction, player_id, page).await
    );
    if leagues.len() > 0 {
        return Ok(leagues);
    }
    service_error!(404, SE::NotFoundError { message: "No leagues found hosted by player.".into()})
}

pub async fn get_average_league_age(
    transaction: &mut PgConnection,
    league_id: i32,
    _user_id: i32,
) -> ServiceResponse<u8> {
    let all_players_in_league = x_u_res_db_or_sr!(
        player_dao::get_all_players_in_league(transaction, league_id, LeaguePlayerStatus::Joined).await
    );
    let (mut age_total, mut amount): (u8, u8) = (0, 0);
    for player in all_players_in_league {
        age_total = age_total
            + (Utc::now()
            .signed_duration_since(player.birth_date)
                .num_days()
                / 365) as u8;
        amount += 1;
    }
    Ok(age_total / amount)
}
