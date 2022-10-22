use chrono::NaiveDate;
use err::MessageResource;

pub fn validate_name(_name: &String) -> Result<(), MessageResource> {
    Ok(())
}
pub fn validate_birth_date(_birth_date: &NaiveDate) -> Result<(), MessageResource> {
    //TODO: Validate date is not in the future & that it's at least a certain age.
    Ok(())
}
pub fn validate_country(_country: &String) -> Result<(), MessageResource> {
    //TODO: Validate country exists?
    Ok(())
}
pub fn validate_city(_city: &String) -> Result<(), MessageResource> {
    //TODO: Validate city?
    Ok(())
}
/// Once an id_number is verified it cannot be changed.
pub fn validate_identification_number(
    _identification_number: &String,
) -> Result<(), MessageResource> {
    //TODO: Validate the id?
    Ok(())
}
pub fn validate_bio(_bio: &String) -> Result<(), MessageResource> {
    //TODO: Validate bio by length & weird chars
    Ok(())
}
pub fn validate_profile_picture_url(_profile_picture_url: &String) -> Result<(), MessageResource> {
    //TODO: Check if it's a valid URL
    //TODO: Download the image?
    Ok(())
}
