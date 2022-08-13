use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub name: String,
    pub birth_date: NaiveDateTime,
    pub country: String,
    pub city: String,
    pub identification_number: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_url: Option<String>,
    pub id_verified: bool,
    pub phone_number_verified: bool,

}

impl Player{
    pub fn new() -> Player {
        Player { id: -1, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), name: "".to_string(), birth_date: Utc::now().naive_utc(), country: "".to_string(), city: "".to_string(), identification_number: None, bio: None, profile_picture_url: None, id_verified: false, phone_number_verified: false }
    }
}