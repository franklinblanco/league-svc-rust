use sqlx::{MySqlPool, mysql::MySqlQueryResult};

use crate::domain::league::League;


pub async fn insert_league(conn: &MySqlPool, league: League) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query_file!("sql/league/insert.sql", league.owner_id, league.sport_id, league.state, league.visibility, league.date_and_time, league.cost_to_join, league.currency, league.max_players, league.description).execute(conn).await
}

pub async fn get_league_with_id(conn: &MySqlPool, league_id: i32) -> Result<Option<League>, sqlx::Error>{
    sqlx::query_file_as!(League, "sql/league/get.sql", league_id).fetch_optional(conn).await
}

pub async fn update_league_with_id(conn: &MySqlPool, league: League) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/league/update.sql", league.owner_id, league.sport_id, league.state, league.visibility, league.date_and_time, league.cost_to_join, league.currency, league.max_players, league.description, league.id).execute(conn).await
}