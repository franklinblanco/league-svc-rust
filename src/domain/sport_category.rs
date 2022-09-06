use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SportCategory {
    pub id: u32,
    pub name: String,
}
impl SportCategory {
    pub fn new() -> SportCategory {
        SportCategory { id: 0, name: "".to_string() }
    }
}