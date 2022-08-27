use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerForCreationDto {
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub password: String,
    pub name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: NaiveDateTime,
    pub country: String,
    pub city: String,
}
