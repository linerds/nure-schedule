use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    pub id: i32,
    pub full_name: String,
    pub short_name: String,
    pub department_id: i32,
}
