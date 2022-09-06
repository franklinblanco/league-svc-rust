use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Place{
    pub id: u32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub name: String,
    pub sport_id: u32,
    pub country: String,
    pub state: Option<String>,
    pub city: String,
    pub address: String,
    pub maps_url: Option<String>,
    pub contact_number: Option<String>,
    pub picture_url: Option<String>
}
impl Place {
    pub fn new() -> Place{
        Place { id: 0, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), name: "".to_string(), sport_id: 0, country: "".to_string(), state: None, city: "".to_string(), address: "".to_string(), maps_url: None, contact_number: None, picture_url: None }
    }
}