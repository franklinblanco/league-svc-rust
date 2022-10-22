use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::{
    enums::league_player_status::LeaguePlayerStatus, league_player::LeaguePlayer,
};
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

pub async fn insert_league_player(
    conn: &MySqlPool,
    league_player: &LeaguePlayer,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/league_player/insert.sql",
            league_player.league_id,
            league_player.player_id,
            league_player.status
        )
        .execute(conn)
        .await
    )
}

pub async fn update_league_player_status(
    conn: &MySqlPool,
    league_id: u32,
    player_id: u32,
    status: &LeaguePlayerStatus,
) -> Result<MySqlQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/league_player/update.sql",
            status.to_string(),
            league_id,
            player_id
        )
        .execute(conn)
        .await
    )
}

pub async fn get_league_player_by_id(
    conn: &MySqlPool,
    id: u32,
) -> Result<LeaguePlayer, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get.sql", id)
            .fetch_one(conn)
            .await
    )
}

pub async fn get_league_players_by_player_id_and_league_id(
    conn: &MySqlPool,
    league_id: u32,
    player_id: u32,
) -> Result<Vec<LeaguePlayer>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            LeaguePlayer,
            "sql/league_player/get_by_league_and_player.sql",
            league_id,
            player_id
        )
        .fetch_all(conn)
        .await
    )
}

//Obsolete code
/*pub async fn get_league_players_by_league_id(conn: &MySqlPool, league_id: u32,) -> Result<Vec<LeaguePlayer>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get_by_league.sql", league_id).fetch_all(conn).await)
}

pub async fn get_league_players_by_player_id(conn: &MySqlPool, player_id: u32,) -> Result<Vec<LeaguePlayer>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get_by_player.sql", player_id).fetch_all(conn).await)
}*/
