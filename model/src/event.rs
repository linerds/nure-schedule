use crate::impl_borrow;

use super::Id;

use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Debug)]
pub struct Event {
    pub id: Id,
    pub starts_at: i64,
    pub ends_at: i64,
    pub kind: EventKind,
    pub count: u8,
    pub subject: Id,
    pub auditorium: Id,
    pub groups: HashSet<Id>,
    pub teachers: HashSet<Id>,
}

impl_borrow!(Event);

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.starts_at.cmp(&other.starts_at)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// #[non_exhaustive] // just in case
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EventKind {
    Lecture,
    PracticalWork,
    LaboratoryWork,
    Consultation,
    /// Залік
    FinalTest,
    Exam,
    CourseWork,
    Unknown,
}
