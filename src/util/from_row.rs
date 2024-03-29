use league_types::dto::player_metadata::PlayerMetadata;
use sqlx::{postgres::PgRow, Row};

pub fn player_metadata_from_row(row: &PgRow) -> Result<PlayerMetadata, sqlx::Error> {
    let id: i32 = row.try_get("id")?;
    let name: String = row.try_get("name")?;
    let profile_picture_url: Option<String> = row.try_get("profile_picture_url")?;
    Ok(PlayerMetadata {
        id,
        name,
        profile_picture_url,
    })
}
