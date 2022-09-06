use serde::{Serialize, Deserialize};


/// This DTO can be used to add to trusted list or to remove. Depends on the endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrustRequestDto {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[serde(rename = "trusterId")]
    pub truster_id: u32,
    #[serde(rename = "trusteeId")]
    pub trustee_id: u32,
}