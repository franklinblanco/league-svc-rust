use sqlx::{MySqlPool, mysql::MySqlQueryResult};

use crate::domain::{player::Player};

pub async fn insert_player(conn: &MySqlPool, player: Player) -> Result<MySqlQueryResult, sqlx::Error>{
    sqlx::query_file!("sql/player/insert.sql", player.name, player.birth_date, player.country, player.city, player.identification_number, player.bio, player.profile_picture_url, player.id_verified, player.phone_number_verified).execute(conn).await
}

pub async fn get_player_with_id(conn: &MySqlPool, player_id: i32) -> Result<Option<Player>, sqlx::Error>{
    sqlx::query_file_as!(Player, "sql/player/get.sql", player_id).fetch_optional(conn).await
}

pub async fn update_player_with_id(conn: &MySqlPool, player: Player) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query_file!("sql/player/update.sql", player.name, player.birth_date, player.country, player.city, player.identification_number, player.bio, player.profile_picture_url, player.id_verified, player.phone_number_verified, player.id).execute(conn).await
}