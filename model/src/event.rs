use proc::PartialBorrow;
use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Debug, PartialBorrow)]
pub struct Event {
    #[borrow_id]
    pub id: i64,
    pub starts_at: i64,
    pub ends_at: i64,
    pub kind: EventKind,
    pub count: u8,
    pub subject: i64,
    pub auditorium: i64,
    pub groups: HashSet<i64>,
    pub teachers: HashSet<i64>,
}

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
