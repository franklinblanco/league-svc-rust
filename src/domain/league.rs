use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::types::Decimal;
use actix_web_utils::enums::error::Error;

use crate::dto::league::LeagueForCreationDto;


#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub id: u32,
    pub owner_id: u32,
    pub sport_id: u32,
    pub place_id: u32,
    pub time_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    /// State as in: Is the league open or closed? Not the geographical sense.
    pub state: String,
    pub visibility: String,
    /// When is the league happening?
    pub date_and_time: NaiveDateTime, //TODO: Switch from NaiveDateTime to TimeZones
    /// This will be stored as a Decimal in the database but the actual input from the user
    /// will not be in rust_decimal::Decimal type.
    pub cost_to_join: Decimal,
    /// This is a string because it's actually meaningless right now. 
    /// We're not taking payments so this doesn't matter, it's just what the user wants.
    pub currency: Option<String>,
    pub max_players: u32,
    pub description: Option<String>
}

impl League {
    pub fn new() -> League {
        League { id: 0, owner_id: 0, sport_id: 0, place_id: 0, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), state: "".to_string(), visibility: "".to_string(), date_and_time: Utc::now().naive_utc(), cost_to_join: Decimal::new(0, 0), currency: None, max_players: 0, description: None }
    }
    pub fn new_from_league_for_creation_dto(league_dto: LeagueForCreationDto) -> League {
        League { 
            id: 0, owner_id: league_dto.user_id, sport_id: league_dto.sport_id, place_id:league_dto.place_id, time_created: Utc::now().naive_utc(), last_updated: Utc::now().naive_utc(), state: LeagueState::Open.to_string(),
            visibility: match league_dto.visibility {
                Some(visibility) => visibility.to_string(),
                None => LeagueVisibility::Public.to_string(),
        },
        date_and_time: league_dto.date_and_time, cost_to_join: league_dto.cost_to_join, currency: league_dto.currency, max_players: league_dto.max_players, description: league_dto.description 
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LeagueState {
    /// Taking new players
    Open,
    /// No more people
    Closed
}
impl Display for LeagueState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeagueState::Open => write!(f, "Open"),
            LeagueState::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LeagueVisibility {
    /// Open to anyone, anyone can join
    Public,
    /// People request to join
    Private,
    /// Only owner can see
    Unlisted
}

impl Display for LeagueVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LeagueVisibility::Public => write!(f, "Public"), 
            LeagueVisibility::Private => write!(f, "Private"),
            LeagueVisibility::Unlisted => write!(f, "Unlisted"),
        }
    }
}
impl FromStr for LeagueVisibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Public" => Ok(LeagueVisibility::Public),
            "Private" => Ok(LeagueVisibility::Private),
            "Unlisted" => Ok(LeagueVisibility::Unlisted),
            _ => Err(Error::Unspecified)
        }
    }
}
