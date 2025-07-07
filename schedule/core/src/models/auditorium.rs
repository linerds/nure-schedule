use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auditorium {
    pub id: i32,
    pub name: String,
    pub floor: i8,
    pub has_power: bool,
    pub building_id: String,
}
