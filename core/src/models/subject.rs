use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Subject {
    id: i32,
    title: String,
    brief: String,
}
