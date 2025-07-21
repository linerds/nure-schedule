use crate::models::{AuditoriumBase, EventKind, Group, Subject, Teacher};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i32,

    #[serde(rename = "startedAt")]
    pub start_at: DateTime<Utc>,
    #[serde(rename = "endedAt")]
    pub end_at: DateTime<Utc>,

    /// The sequence number of the current class
    #[serde(rename = "numberPair")]
    pub number: u8,
    #[serde(rename = "type")]
    pub kind: EventKind,

    pub subject: Subject,
    // FIXME: those below are broken and need their own separate stripped types.
    pub auditorium: AuditoriumBase,
    pub groups: Vec<Group>,
    pub teachers: Vec<Teacher>,
}
