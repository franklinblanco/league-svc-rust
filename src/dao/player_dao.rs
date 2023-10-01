use league_types::domain::{player::Player, enums::league_player_status::LeaguePlayerStatus};
use sqlx::PgConnection;

pub async fn insert_player(
    conn: &mut PgConnection,
    player: Player,
) -> Result<Player, sqlx::Error> {
    sqlx::query_file_as!(
        Player,
        "sql/player/insert.sql",
        player.time_created,
        player.name,
        player.birth_date,
        player.country,
        player.city,
        player.identification_number,
        player.bio,
        player.profile_picture_url,
        player.id_verified,
        player.phone_number_verified
    )
    .fetch_one(conn)
    .await
}

pub async fn get_player_with_id(
    conn: &mut PgConnection,
    player_id: i32,
) -> Result<Option<Player>, sqlx::Error> {
    sqlx::query_file_as!(Player, "sql/player/get.sql", player_id)
        .fetch_optional(conn)
        .await
}

pub async fn update_player_with_id(
    conn: &mut PgConnection,
    player: Player,
) -> Result<Player, sqlx::Error> {
    sqlx::query_file_as!(
        Player,
        "sql/player/update.sql",
        player.last_updated,
        player.name,
        player.birth_date,
        player.country,
        player.city,
        player.identification_number,
        player.bio,
        player.profile_picture_url,
        player.id_verified,
        player.phone_number_verified,
        player.id
    )
    .fetch_one(conn)
    .await
}

pub async fn get_all_players_in_league(
    conn: &mut PgConnection,
    league_id: i32,
    lp_status: LeaguePlayerStatus
) -> Result<Vec<Player>, sqlx::Error> {
    sqlx::query_file_as!(Player, "sql/player/get_by_league_player.sql", league_id, lp_status.to_string())
        .fetch_all(conn)
        .await
}

pub async fn get_all_trusted_players(
    conn: &mut PgConnection,
    player_id: i32,
) -> Result<Vec<Player>, sqlx::Error> {
    sqlx::query_file_as!(
        Player,
        "sql/player/get_trusted_players_by_truster.sql",
        player_id
    )
    .fetch_all(conn)
    .await
}

pub async fn get_all_players_that_trust_player(
    conn: &mut PgConnection,
    player_id: i32,
) -> Result<Vec<Player>, sqlx::Error> {
    sqlx::query_file_as!(
        Player,
        "sql/player/get_players_that_trust_trustee.sql",
        player_id
    )
    .fetch_all(conn)
    .await
}


pub async fn get_players_bulk(
    conn: &mut PgConnection,
    player_ids: Vec<i32>,
) -> Result<Vec<Player>, sqlx::Error> {
    sqlx::query_file_as!(Player, "sql/player/get_players_with_ids.sql", &player_ids)
        .fetch_all(conn)
        .await
}
