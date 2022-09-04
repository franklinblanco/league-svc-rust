use actix_web_utils::{extensions::typed_response::TypedHttpResponse, unwrap_or_return_handled_error, dtos::message::MessageResource};
use dev_communicators::middleware::user_svc::user_service::authenticate_user_with_token;
use dev_dtos::dtos::user::user_dtos::UserForAuthenticationDto;
use reqwest::Client;
use sqlx::MySqlPool;

use crate::{dto::league_player::JoinRequest, domain::{league::{LeagueVisibility, League}, league_player::LeaguePlayer, enums::league_player_status::LeaguePlayerStatus, player::Player}, util::env_util::APP_NAME, dao::{player_dao::{get_player_with_id, self}, league_dao, league_player_dao}};


pub async fn request_to_join_league(conn: &MySqlPool, client: &Client, join_req: JoinRequest) -> TypedHttpResponse<LeaguePlayer> {
    let league = match unwrap_or_return_handled_error!(league_dao::get_league_with_id(conn, join_req.league_id).await, LeaguePlayer) {
        Some(league) => league,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("League not found with given id.")),
    };
    let user_for_auth = UserForAuthenticationDto { app: APP_NAME.to_owned(), id: join_req.user_id.to_string(), token: join_req.auth_token.clone()};
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    match unwrap_or_return_handled_error!(get_player_with_id(conn, user.id).await, LeaguePlayer) {
        Some(player) => player,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("Player profile not found.")),
    };
    let mut league_player_to_insert = LeaguePlayer::new_from_join_request(join_req);
    let persisted_league_players = unwrap_or_return_handled_error!(league_player_dao::get_league_players_by_player_id_and_league_id(conn, league_player_to_insert.league_id, league_player_to_insert.player_id).await, LeaguePlayer);
    if !persisted_league_players.is_empty() {
        return TypedHttpResponse::return_standard_error(400, MessageResource::new_from_str("Your join request for this league already exists."));
    }
    let join_request_status = match unwrap_or_return_handled_error!(400, league.visibility.parse::<LeagueVisibility>(), LeaguePlayer) {
        LeagueVisibility::Public => LeaguePlayerStatus::Joined,
        LeagueVisibility::Private => LeaguePlayerStatus::Requested,
        LeagueVisibility::Unlisted => LeaguePlayerStatus::Denied,
    };
    league_player_to_insert.status = join_request_status.to_string();
    let last_insert_id = unwrap_or_return_handled_error!(league_player_dao::insert_league_player(conn, &league_player_to_insert).await, LeaguePlayer).last_insert_id() as i32;
    unwrap_or_return_handled_error!(500, 200, league_player_dao::get_league_player_by_id(conn, last_insert_id).await, LeaguePlayer);
}



pub async fn get_league_request_status(conn: &MySqlPool, client: &Client, join_req: JoinRequest) -> TypedHttpResponse<LeaguePlayer> {
    let user_for_auth = UserForAuthenticationDto { app: APP_NAME.to_owned(), id: join_req.user_id.to_string(), token: join_req.auth_token.clone()};
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    match unwrap_or_return_handled_error!(league_player_dao::get_league_players_by_player_id_and_league_id(conn, join_req.league_id, user.id).await, LeaguePlayer).get(0) {
        Some(league_player) => TypedHttpResponse::return_standard_response(200, league_player.clone()),
        None => TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("LeaguePlayer not found with given ids.")),
    }
}



pub async fn change_league_request_status(conn: &MySqlPool, client: &Client, new_status: LeaguePlayerStatus, join_req: JoinRequest) -> TypedHttpResponse<LeaguePlayer> {
    let league = match unwrap_or_return_handled_error!(league_dao::get_league_with_id(conn, join_req.league_id).await, LeaguePlayer) {
        Some(league) => league,
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("League not found with given id.")),
    };
    let user_for_auth = UserForAuthenticationDto { app: APP_NAME.to_owned(), id: join_req.user_id.to_string(), token: join_req.auth_token.clone()};
    let user = unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        LeaguePlayer
    );
    if league.owner_id != user.id {
        return TypedHttpResponse::return_standard_error(401, MessageResource::new_from_str("You don't own this league..."))
    }
    let mut persisted_league_player = match unwrap_or_return_handled_error!(league_player_dao::get_league_players_by_player_id_and_league_id(conn, join_req.league_id, join_req.user_id).await, LeaguePlayer).get(0) {
        Some(league_player) => league_player.clone(),
        None => return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("LeaguePlayer not found with given ids.")),
    };
    unwrap_or_return_handled_error!(league_player_dao::update_league_player_status(conn, user.id, &new_status).await, LeaguePlayer);
    persisted_league_player.status = new_status.to_string();
    TypedHttpResponse::return_standard_response(200, persisted_league_player)
}


pub async fn get_all_leagues_player_has_applied_to(conn: &MySqlPool, client: &Client, join_req: JoinRequest, page: i32) -> TypedHttpResponse<Vec<League>> {
    let user_for_auth = UserForAuthenticationDto { app: APP_NAME.to_owned(), id: join_req.user_id.to_string(), token: join_req.auth_token.clone()};
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<League>
    );
    let from_row = (page * 20) - 20;
    let to_row = page * 20;
    let resulting_leagues = unwrap_or_return_handled_error!(league_dao::get_all_leagues_player_has_applied_to(conn, join_req.user_id, from_row, to_row).await, Vec<League>);
    if resulting_leagues.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, resulting_leagues);
    }
    return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No leagues found with player join requests."));
}



pub async fn get_all_players_in_league(conn: &MySqlPool, client: &Client, join_req: JoinRequest) -> TypedHttpResponse<Vec<Player>> {
    let user_for_auth = UserForAuthenticationDto { app: APP_NAME.to_owned(), id: join_req.user_id.to_string(), token: join_req.auth_token.clone()};
    unwrap_or_return_handled_error!(
        401,
        authenticate_user_with_token(client, &user_for_auth).await,
        Vec<Player>
    );
    let resulting_players: Vec<Player> = unwrap_or_return_handled_error!(player_dao::get_all_players_in_league(conn, join_req.league_id).await, Vec<Player>).into_iter().map(|player| player.clear_all_sensitive_fields()).collect();
    if resulting_players.len() > 0 {
        return TypedHttpResponse::return_standard_response(200, resulting_players);
    }
    return TypedHttpResponse::return_standard_error(404, MessageResource::new_from_str("No players found with join requests to the league specified."));
}