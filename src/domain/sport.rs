use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sport{
    pub id: i32,
    pub name: String,
    pub category_id: i32
}
impl Sport{
    pub fn new() -> Sport{
        Sport { id: -1, name: "".to_string(), category_id: -1 }
    }
}