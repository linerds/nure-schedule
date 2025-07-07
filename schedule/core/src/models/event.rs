use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{Auditorium, EventType, Group, Subject, Teacher};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i32,

    pub started_at: NaiveDateTime,
    pub ended_at: NaiveDateTime,

    pub number_pair: u8,
    #[serde(rename = "type")]
    pub event_type: EventType,

    pub auditorium: Auditorium,
    pub subject: Subject,
    pub teachers: Vec<Teacher>,
    pub groups: Vec<Group>,
}
