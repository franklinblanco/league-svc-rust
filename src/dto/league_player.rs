use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRequest {
    #[serde(rename = "leagueId")]
    pub league_id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "authToken")]
    pub auth_token: String,
}