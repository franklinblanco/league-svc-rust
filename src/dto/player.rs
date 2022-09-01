use chrono::{NaiveDate};
use dev_dtos::{dtos::user::user_dtos::UserForCreationDto, domain::user::credential_type::CredentialType};
use serde::{Serialize, Deserialize};

use crate::util::env_util::APP_NAME;

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

impl PlayerForCreationDto {
    pub fn convert_player_into_user_for_creation(player: &PlayerForCreationDto) -> UserForCreationDto{
        UserForCreationDto { app: APP_NAME.to_owned(), credential: player.phone_number.clone(), credential_type: CredentialType::PhoneNumber, password: player.password.clone(), name: player.name.clone() }
    }
}