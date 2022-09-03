use std::{fmt::Display, str::FromStr};

use serde::{Serialize, Deserialize};
use actix_web_utils::enums::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LeaguePlayerStatus {
    Denied,
    Joined,
    Requested,
    Kicked
}
impl Display for LeaguePlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeaguePlayerStatus::Denied => write!(f, "Denied"),
            LeaguePlayerStatus::Joined => write!(f, "Joined"),
            LeaguePlayerStatus::Requested => write!(f, "Requested"),
            LeaguePlayerStatus::Kicked => write!(f, "Kicked"),
        }
    }
}
impl FromStr for LeaguePlayerStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Denied" => Ok(Self::Denied),
            "Joined" => Ok(Self::Joined),
            "Requested" => Ok(Self::Requested),
            "Kicked" => Ok(Self::Kicked),
            _ => Err(Error::Unspecified) //TODO: Create ParseStr error in actix_web_utils
        }
    }
}