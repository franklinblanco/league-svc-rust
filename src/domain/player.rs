use chrono::{NaiveDateTime, Utc, NaiveDate};
use serde::{Serialize, Deserialize};

use crate::dto::player::PlayerForCreationDto;

//TODO: Remove sensitive information from player struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub name: String,
    pub birth_date: NaiveDate,
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
        Player { id: 0, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), name: "".to_string(), birth_date: Utc::now().date_naive(), country: "".to_string(), city: "".to_string(), identification_number: None, bio: None, profile_picture_url: None, id_verified: false, phone_number_verified: false }
    }
    pub fn new_from_creation_dto(player_dto: &PlayerForCreationDto, id: &u32) -> Player {
        Player {  id: *id, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), name: player_dto.name.clone(), birth_date: player_dto.birth_date.clone(), country: player_dto.country.clone(), city: player_dto.city.clone(), identification_number: None, bio: None, profile_picture_url: None, id_verified: false, phone_number_verified: false }
    }
    pub fn clear_all_sensitive_fields(mut self) -> Self {
        self.birth_date = Utc::now().date_naive();
        self.city = "".to_string();
        self.id_verified = false;
        self.identification_number = None;
        self.last_updated = Utc::now().naive_utc();
        self
    }
}