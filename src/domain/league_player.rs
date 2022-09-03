use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::dto::league_player::JoinRequest;

use super::enums::league_player_status::LeaguePlayerStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaguePlayer {
    pub id: i32,
    pub league_id: i32,
    pub player_id: i32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub status: String
}
impl LeaguePlayer {
    pub fn new_empty() -> LeaguePlayer {
        LeaguePlayer { id: 0, league_id: -1, player_id: -1, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), status: "".to_string()}
    }
    pub fn new_from_join_request(join_req: JoinRequest) -> LeaguePlayer {
        LeaguePlayer { id: 0, league_id: join_req.league_id, player_id: join_req.user_id, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), status: LeaguePlayerStatus::Requested.to_string() }
    }
}