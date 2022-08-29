use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerForCreationDto {
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    pub password: String,
    pub name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: NaiveDate,
    pub country: String,
    pub city: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerForUpdateDto {
    pub name: Option<String>,
    #[serde(rename = "birthDate")]
    pub birth_date: Option<NaiveDate>,
    pub country: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "identificationNumber")]
    pub identification_number: Option<String>,
    pub bio: Option<String>,
    #[serde(rename = "profilePictureUrl")]
    pub profile_picture_url: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}