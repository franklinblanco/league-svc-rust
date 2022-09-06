use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sport{
    pub id: u32,
    pub name: String,
    pub category_id: u32
}
impl Sport{
    pub fn new() -> Sport{
        Sport { id: 0, name: "".to_string(), category_id: 0 }
    }
}