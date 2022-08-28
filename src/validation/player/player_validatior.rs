use crate::domain::player::Player;

pub fn validate_player(_player: Player) -> Result<(), ()> {
    Ok(())
}
pub fn validate_name() {

}
pub fn validate_birth_date() {

}
pub fn validate_country() {

}
pub fn validate_city() {

}
/// Once an id_number is verified it cannot be changed.
pub fn validate_identification_number() {

}
pub fn validate_bio() {

}
pub fn validate_profile_picture_url(_profile_picture_url: String) -> Result<(), ()>{
    //TODO: Check if it's a valid URL
    //TODO: Download the image?
    Ok(())
}