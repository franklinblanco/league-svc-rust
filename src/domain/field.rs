use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::types::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: u32,
    pub place_id: u32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub country: String,
    pub city: String,
    pub name: String,
    pub price_per_hour: Decimal,
    pub currency: String,
    pub description: Option<String>,
}

impl Field {
    pub fn new() -> Field {
        Field { id: 0, place_id: 0, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), country: "".to_string(), city: "".to_string(), name: "".to_string(), price_per_hour: Decimal::new(0, 0), currency: "".to_string(), description: None }
    }
}