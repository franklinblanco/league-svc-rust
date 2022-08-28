use crate::{dto::player_dtos::PlayerForUpdateDto, domain::player::Player};


pub fn update_player_object(update_player: PlayerForUpdateDto, _existing_player: Player) -> Player {
    //  Bunch of if statements?
    match update_player.name {
        Some(_) => todo!(),
        None => todo!(),
    }
}