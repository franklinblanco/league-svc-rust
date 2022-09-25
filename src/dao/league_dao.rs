use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::league::{League, LeagueVisibility};
use sqlx::{MySqlPool, mysql::MySqlQueryResult};

pub async fn insert_league(conn: &MySqlPool, league: League) -> Result<MySqlQueryResult, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/league/insert.sql", league.owner_id, league.sport_id, league.place_id, league.state, league.visibility, league.date_and_time, league.cost_to_join, league.currency, league.max_players, league.description).execute(conn).await)
}

pub async fn get_league_with_id(conn: &MySqlPool, league_id: u32) -> Result<Option<League>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(League, "sql/league/get.sql", league_id).fetch_optional(conn).await)
}

pub async fn get_leagues_by_country_limited_to(conn: &MySqlPool, country: String, from_row: u32, to_row: u32) -> Result<Vec<League>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(League, "sql/league/get_by_country_limited.sql", country, from_row, to_row).fetch_all(conn).await)
}

pub async fn get_leagues_by_in_place_limited_to(conn: &MySqlPool, place_id: u32, from_row: u32, to_row: u32) -> Result<Vec<League>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(League, "sql/league/get_by_place_limited.sql", place_id, from_row, to_row).fetch_all(conn).await)
}

/// Only gets public & private leagues, organized by time_created to display newer leagues first. Unlisted leagues don't show.
pub async fn get_leagues_by_player_limited_to(conn: &MySqlPool, player_id: u32, from_row: u32, to_row: u32) -> Result<Vec<League>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(League, "sql/league/get_by_player_limited.sql", player_id, LeagueVisibility::Unlisted.to_string(), from_row, to_row).fetch_all(conn).await)
}

pub async fn update_league_with_id(conn: &MySqlPool, league: League) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/league/update.sql", league.owner_id, league.sport_id, league.place_id, league.state, league.visibility, league.date_and_time, league.cost_to_join, league.currency, league.max_players, league.description, league.id).execute(conn).await)
}

pub async fn get_all_leagues_player_has_applied_to(conn: &MySqlPool, player_id: u32, from_row: u32, to_row: u32) -> Result<Vec<League>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(League, "sql/league/get_by_league_player.sql", player_id, from_row, to_row).fetch_all(conn).await)
}