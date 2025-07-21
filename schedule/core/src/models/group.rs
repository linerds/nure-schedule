use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i32,
    pub name: String,

    // WARNING: `Event` expects a `Group { id, name }`. Change those two with caution
    pub direction_id: Option<i32>,
    pub speciality_id: Option<i32>,
}
