use actix_web_utils::{extensions::generic_error::GenericError, wrap_generic_error_in_wrapper};
use league_types::domain::league::{League, LeagueVisibility};
use sqlx::{postgres::PgQueryResult, PgPool};

pub async fn insert_league(
    conn: &PgPool,
    league: League,
) -> Result<League, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            League,
            "sql/league/insert.sql",
            league.owner_id,
            league.sport_id,
            league.place_id,
            league.time_created,
            league.state,
            league.visibility,
            league.date_and_time,
            league.cost_to_join,
            league.currency,
            league.max_players,
            league.description
        )
        .fetch_one(conn)
        .await
    )
}

pub async fn get_league_with_id(
    conn: &PgPool,
    league_id: i32,
) -> Result<Option<League>, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(League, "sql/league/get.sql", league_id)
            .fetch_optional(conn)
            .await
    )
}

pub async fn get_leagues_by_country_limited_to(
    conn: &PgPool,
    country: String,
    page: i64,
) -> Result<Vec<League>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            League,
            "sql/league/get_by_country_limited.sql",
            country,
            offset
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn get_leagues_by_in_place_limited_to(
    conn: &PgPool,
    place_id: i32,
    page: i64,
) -> Result<Vec<League>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            League,
            "sql/league/get_by_place_limited.sql",
            place_id,
            offset
        )
        .fetch_all(conn)
        .await
    )
}

/// Only gets public & private leagues, organized by time_created to display newer leagues first. Unlisted leagues don't show.
pub async fn get_leagues_by_player_limited_to(
    conn: &PgPool,
    player_id: i32,
    page: i64,
) -> Result<Vec<League>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            League,
            "sql/league/get_by_player_limited.sql",
            player_id,
            LeagueVisibility::Unlisted.to_string(),
            offset
        )
        .fetch_all(conn)
        .await
    )
}

pub async fn update_league_with_id(
    conn: &PgPool,
    league: League,
) -> Result<PgQueryResult, GenericError<sqlx::Error>> {
    wrap_generic_error_in_wrapper!(
        sqlx::query_file!(
            "sql/league/update.sql",
            league.owner_id,
            league.sport_id,
            league.place_id,
            league.last_updated,
            league.state,
            league.visibility,
            league.date_and_time,
            league.cost_to_join,
            league.currency,
            league.max_players,
            league.description,
            league.id
        )
        .execute(conn)
        .await
    )
}

pub async fn get_all_leagues_player_has_applied_to(
    conn: &PgPool,
    player_id: i32,
    page: i64,
) -> Result<Vec<League>, GenericError<sqlx::Error>> {
    let offset = (page - 1) * 25;
    wrap_generic_error_in_wrapper!(
        sqlx::query_file_as!(
            League,
            "sql/league/get_by_league_player.sql",
            player_id,
            offset
        )
        .fetch_all(conn)
        .await
    )
}
