use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place{
    pub id: i32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub name: String,
    pub sport_id: i32,
    pub address: Option<String>,
    pub maps_url: Option<String>,
    pub contact_number: Option<String>,
    pub picture_url: Option<String>
}
impl Place {
    pub fn new() -> Place{
        Place { id: -1, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), name: "".to_string(), sport_id: -1, address: None, maps_url: None, contact_number: None, picture_url: None }
    }
}