use chrono::Utc;
use league_types::domain::{
    enums::league_player_status::LeaguePlayerStatus, league_player::LeaguePlayer,
};
use sqlx::PgConnection;

pub async fn insert_league_player(
    conn: &mut PgConnection,
    league_player: &LeaguePlayer,
) -> Result<LeaguePlayer, sqlx::Error> {
        sqlx::query_file_as!(
            LeaguePlayer,
            "sql/league_player/insert.sql",
            league_player.league_id,
            league_player.player_id,
            league_player.time_created,
            league_player.status as _
        )
        .fetch_one(conn)
        .await
}

pub async fn update_league_player_status(
    conn: &mut PgConnection,
    league_player_id: i32,
    status: &LeaguePlayerStatus,
) -> Result<LeaguePlayer, sqlx::Error> {
    let update_time = Utc::now();
    sqlx::query_file_as!(
        LeaguePlayer,
        "sql/league_player/update.sql",
        update_time,
        status as _,
        league_player_id,
    )
    .fetch_one(conn)
    .await
}

pub async fn get_league_player_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> Result<LeaguePlayer, sqlx::Error> {
    sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get.sql", id)
        .fetch_one(conn)
        .await
}

pub async fn get_league_players_by_player_id_and_league_id(
    conn: &mut PgConnection,
    league_id: i32,
    player_id: i32,
) -> Result<Vec<LeaguePlayer>, sqlx::Error> {
    sqlx::query_file_as!(
        LeaguePlayer,
        "sql/league_player/get_by_league_and_player.sql",
        league_id,
        player_id
    )
    .fetch_all(conn)
    .await
}

//Obsolete code
/*pub async fn get_league_players_by_league_id(conn: &mut PgConnection, league_id: i32,) -> Result<Vec<LeaguePlayer>, sqlx::Error> {
    sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get_by_league.sql", league_id).fetch_all
}

pub async fn get_league_players_by_player_id(conn: &mut PgConnection, player_id: i32,) -> Result<Vec<LeaguePlayer>, sqlx::Error> {
    sqlx::query_file_as!(LeaguePlayer, "sql/league_player/get_by_player.sql", player_id).fetch_all
}*/
