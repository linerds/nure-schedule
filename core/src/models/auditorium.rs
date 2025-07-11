use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AuditoriumBase {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuditoriumFull {
    pub id: i32,
    pub name: String,
    pub floor: i8,
    pub has_power: bool,
    pub building_id: String,
}

impl From<AuditoriumFull> for AuditoriumBase {
    fn from(value: AuditoriumFull) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
