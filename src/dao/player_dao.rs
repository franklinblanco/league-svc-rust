use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::player::Player;
use sqlx::{MySqlPool, mysql::MySqlQueryResult};

pub async fn insert_player(conn: &MySqlPool, player: Player) -> Result<MySqlQueryResult, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/player/insert.sql", player.id, player.name, player.birth_date, player.country, player.city, player.identification_number, player.bio, player.profile_picture_url, player.id_verified, player.phone_number_verified).execute(conn).await)
}

pub async fn get_player_with_id(conn: &MySqlPool, player_id: u32) -> Result<Option<Player>, GenericError<sqlx::Error>>{
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Player, "sql/player/get.sql", player_id).fetch_optional(conn).await)
}

pub async fn update_player_with_id(conn: &MySqlPool, player: Player) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file!("sql/player/update.sql", player.name, player.birth_date, player.country, player.city, player.identification_number, player.bio, player.profile_picture_url, player.id_verified, player.phone_number_verified, player.id).execute(conn).await)
}

pub async fn get_all_players_in_league(conn: &MySqlPool, league_id: u32) -> Result<Vec<Player>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Player, "sql/player/get_by_league_player.sql", league_id).fetch_all(conn).await)
}

pub async fn get_all_trusted_players(conn: &MySqlPool, player_id: u32) -> Result<Vec<Player>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Player, "sql/player/get_trusted_players_by_truster.sql", player_id).fetch_all(conn).await)
}

pub async fn get_all_players_that_trust_player(conn: &MySqlPool, player_id: u32) -> Result<Vec<Player>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(Player, "sql/player/get_players_that_trust_trustee.sql", player_id).fetch_all(conn).await)
}