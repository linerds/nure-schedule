use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    pub id: i32,
    pub full_name: String,
    pub short_name: String,

    // FIXME: Event does not provide this field
    pub department_id: i32,
}
