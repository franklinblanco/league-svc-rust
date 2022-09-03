use chrono::NaiveDateTime;
use serde::{Deserialize};
use sqlx::types::Decimal;

use crate::domain::league::LeagueVisibility;

#[derive(Debug, Deserialize, Clone)]
pub struct LeagueForCreationDto{
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "sportId")]
    pub sport_id: i32,
    #[serde(rename = "placeId")]
    pub place_id: i32,
    pub visibility: Option<LeagueVisibility>,
    #[serde(rename = "dateAndTime")]
    pub date_and_time: NaiveDateTime,
    #[serde(rename = "costToJoin")]
    pub cost_to_join: Decimal,
    pub currency: Option<String>,
    #[serde(rename = "maxPlayers")]
    pub max_players: i32,
    pub description: Option<String>
}

