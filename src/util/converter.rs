use actix_web_utils::dtos::message::MessageResource;

use crate::{dto::player_dtos::PlayerForUpdateDto, domain::player::Player, validation::player::player_validatior::{validate_name, validate_bio, validate_profile_picture_url, validate_birth_date, validate_country, validate_identification_number, validate_city}};


pub fn update_player_object(update_player: PlayerForUpdateDto, existing_player: Player) -> Result<Player, Vec<MessageResource>>{
    let mut message_resources: Vec<MessageResource> = Vec::new();
    let mut resulting_player: Player = existing_player.clone();
    //TODO: Log changes
    if let Some(name) = update_player.name { 
        match validate_name(&name) {
            Ok(_) => {resulting_player.name = name},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(bio) = update_player.bio { 
        match validate_bio(&bio) {
            Ok(_) => {resulting_player.bio = Some(bio)},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(profile_picture_url) = update_player.profile_picture_url { 
        match validate_profile_picture_url(&profile_picture_url) {
            Ok(_) => {resulting_player.profile_picture_url = Some(profile_picture_url)},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(birth_date) = update_player.birth_date { 
        match validate_birth_date(&birth_date) {
            Ok(_) => {resulting_player.birth_date = birth_date},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(country) = update_player.country { 
        match validate_country(&country) {
            Ok(_) => {resulting_player.country = country},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(city) = update_player.city { 
        match validate_city(&city) {
            Ok(_) => {resulting_player.city = city},
            Err(message) => message_resources.push(message)
        }
    };

    if let Some(identification_number) = update_player.identification_number { 
        match validate_identification_number(&identification_number) {
            Ok(_) => {resulting_player.identification_number = Some(identification_number)},
            Err(message) => message_resources.push(message)
        }
    };

    if message_resources.len() > 0 {
        return Err(message_resources);
    } else {
        return Ok(resulting_player);
    }
}