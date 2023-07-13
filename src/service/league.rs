use actix_web_utils::{
    extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error,
};
use chrono::Utc;
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use err::MessageResource;
use league_types::{domain::league::League, dto::league::LeagueForCreationDto, APP_NAME};
use reqwest::Client;
use sqlx::PgPool;

use crate::
    dao::{
        league_dao::*,
        player_dao::{self, *},
    };

/// Create a league.
pub async fn create_league(
    conn: &PgPool,
    client: &Client,
    league: LeagueForCreationDto,
) -> TypedHttpResponse<League> {
    let user_auth_dto = UserForAuthenticationDto {
        app: APP_NAME.to_string(),
        id: league.user_id.to_string(),
        token: league.auth_token.clone(),
    };

    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_auth_dto).await,
        League
    );

    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id as i32).await, League) {
        Some(player) => player,
        None => {
            return TypedHttpResponse::return_standard_error(
                404,
                MessageResource::new_from_str("Player profile not found."),
            )
        }
    };

    // TODO: Validation: League time must be in the future
    // TODO: Validate user doesn't have more than 10 leagues open?
    TypedHttpResponse::return_standard_response(200, 
        unwrap_or_return_handled_error!(insert_league(conn, League::from(league)).await, League))
}

/// Used to get a specific league
pub async fn get_league(conn: &PgPool, id: i32) -> TypedHttpResponse<League> {
    match unwrap_or_return_handled_error!(get_league_with_id(conn, id).await, League) {
        Some(league) => TypedHttpResponse::return_standard_response(200, league),
        None => TypedHttpResponse::return_standard_error(
            404,
            MessageResource::new_from_str("League not found."),
        ),
    }
}

/// This route infers the player's area by his country & city.
pub async fn get_open_leagues_in_my_area(
    conn: &PgPool,
    client: &Client,
    user_for_auth: UserForAuthenticationDto,
    page: i64,
) -> TypedHttpResponse<Vec<League>> {
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );
    let player = match unwrap_or_return_handled_error!(
        get_player_with_id(conn, user.id as i32).await,
        Vec<League>
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
        get_leagues_by_country_limited_to(conn, player.country, page).await,
        Vec<League>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No leagues found for your country."),
    )
}

/// This route is used to get leagues from a country
pub async fn get_leagues_in_country(
    conn: &PgPool,
    country: &String,
    page: i64,
) -> TypedHttpResponse<Vec<League>> {
    let res = unwrap_or_return_handled_error!(
        get_leagues_by_country_limited_to(conn, country.clone(), page)
            .await,
        Vec<League>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No leagues found for country."),
    )
}

/// This route is used to get leagues from a country
pub async fn get_leagues_in_place(
    conn: &PgPool,
    place_id: i32,
    page: i64,
) -> TypedHttpResponse<Vec<League>> {
    let res = unwrap_or_return_handled_error!(
        get_leagues_by_in_place_limited_to(conn, place_id, page).await,
        Vec<League>
    );
    if res.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, res);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No leagues found for place."),
    )
}

/// Only shows non unlisted leagues //TODO: Make a new endpoint to get MyLeagues (Only callable by the owner)
pub async fn get_leagues_hosted_by_player(
    conn: &PgPool,
    client: &Client,
    user_for_auth: UserForAuthenticationDto,
    player_id: i32,
    page: i64,
) -> TypedHttpResponse<Vec<League>> {
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );

    let leagues = unwrap_or_return_handled_error!(
        get_leagues_by_player_limited_to(conn, player_id, page).await,
        Vec<League>
    );
    if leagues.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, leagues);
    }
    TypedHttpResponse::return_standard_error(
        404,
        MessageResource::new_from_str("No leagues found for place."),
    )
}

pub async fn get_average_league_age(
    conn: &PgPool,
    client: &Client,
    user_for_auth: UserForAuthenticationDto,
    league_id: i32,
) -> TypedHttpResponse<u8> {
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        u8
    );
    let all_players_in_league = unwrap_or_return_handled_error!(
        player_dao::get_all_players_in_league(conn, league_id).await,
        u8
    );
    let (mut age_total, mut amount): (u8, u8) = (0, 0);
    for player in all_players_in_league {
        age_total = age_total
            + (Utc::now()
                .date_naive()
                .signed_duration_since(player.birth_date)
                .num_days()
                / 365) as u8;
        amount += 1;
    }
    TypedHttpResponse::return_standard_response(200, age_total / amount)
}
