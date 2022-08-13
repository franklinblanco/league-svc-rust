use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::types::Decimal;


#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub id: i32,
    pub owner_id: i32,
    pub sport_id: i32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub visibility: String,
    pub date_and_time: NaiveDateTime,
    pub cost_to_join: Decimal,
    pub currency: String,
    pub max_players: i32,
    pub description: Option<String>
}

impl League {
    pub fn new() -> League {
        League { id: 0, owner_id: -1, sport_id: -1, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), visibility: "".to_string(), date_and_time: Utc::now().naive_utc(), cost_to_join: Decimal::new(0, 0), currency: "".to_string(), max_players: -1, description: None }
    }
}