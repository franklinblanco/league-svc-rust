use actix_web_utils::{extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error, dtos::message::MessageResource};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{domain::league::League, dto::league::LeagueForCreationDto, util::env_util::APP_NAME, dao::{player_dao::{get_player_with_id}, league_dao::{insert_league, get_league_with_id, get_leagues_by_country_limited_to, get_leagues_by_in_place_limited_to, get_leagues_by_player_limited_to},}};

/// Create a league.
pub async fn create_league(conn: &MySqlPool, client: &Client, league: LeagueForCreationDto) -> TypedHttpResponse<League> {
    let user_auth_dto = UserForAuthenticationDto { app: APP_NAME.to_string(), id: league.user_id.to_string(), token: league.auth_token.clone() };
    
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_auth_dto).await,
        League
    );

    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id).await, League) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Player profile not found.")),
    };

    // TODO: Validation: League time must be in the future
    // TODO: Validate user doesn't have more than 10 leagues open?
    let league_query_reuslt = unwrap_or_return_handled_error!(insert_league(conn, League::new_from_league_for_creation_dto(league)).await, League);
    
    match unwrap_or_return_handled_error!(get_league_with_id(conn, league_query_reuslt.last_insert_id() as i32).await, League) {
        Some(league) => TypedHttpResponse::return_standard_response(200, league),
        None => TypedHttpResponse::return_standard_error(500, MessageResource::new_from_str("League not found.")),
    }
}

/// Used to get a specific league
pub async fn get_league(conn: &MySqlPool, id: i32) -> TypedHttpResponse<League> {
    match unwrap_or_return_handled_error!(get_league_with_id(conn, id).await, League) {
        Some(league) => TypedHttpResponse::return_standard_response(200, league),
        None => TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("League not found.")),
    }
}



/// This route infers the player's area by his country & city.
pub async fn get_open_leagues_in_my_area(conn: &MySqlPool, client: &Client, user_for_auth: UserForAuthenticationDto, page: i32) -> TypedHttpResponse<Vec<League>> {
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );
    let player = match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id).await, Vec<League>) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Player profile not found.")),
    };
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;

    let res = unwrap_or_return_handled_error!(get_leagues_by_country_limited_to(conn, player.country, from_row, to_row).await, Vec<League>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No leagues found for your country."))
}



/// This route is used to get leagues from a country
pub async fn get_leagues_in_country(conn: &MySqlPool, country: &String, page: i32,) -> TypedHttpResponse<Vec<League>> {
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;
    let res = unwrap_or_return_handled_error!(get_leagues_by_country_limited_to(conn, country.clone(), from_row, to_row).await, Vec<League>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No leagues found for country."))
}



/// This route is used to get leagues from a country
pub async fn get_leagues_in_place(conn: &MySqlPool, place_id: i32, page: i32,) -> TypedHttpResponse<Vec<League>> {
    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;
    let res = unwrap_or_return_handled_error!(get_leagues_by_in_place_limited_to(conn, place_id, from_row, to_row).await, Vec<League>);
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No leagues found for place."))
}



/// Only shows non unlisted leagues //TODO: Make a new endpoint to get MyLeagues (Only callable by the owner)
pub async fn get_leagues_hosted_by_player(conn: &MySqlPool, client: &Client, user_for_auth: UserForAuthenticationDto, player_id: i32, page: i32) -> TypedHttpResponse<Vec<League>> {    
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );

    // Code to get the fromRow and the ToRow numbers out of a single page number
    let from_row = (page * 20) - 20;
    let to_row = page * 20;

    let leagues = unwrap_or_return_handled_error!(get_leagues_by_player_limited_to(conn, player_id, from_row, to_row).await, Vec<League>);
    if leagues.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, leagues);
    }
    TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No leagues found for place."))
}