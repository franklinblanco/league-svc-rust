use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SportCategory {
    pub id: i32,
    pub name: String,
}
impl SportCategory {
    pub fn new() -> SportCategory {
        SportCategory { id: -1, name: "".to_string() }
    }
}