use std::collections::BTreeMap;

pub type Groups = BTreeMap<i64, Group>;
pub type Teachers = BTreeMap<i64, Teacher>;
pub type Subjects = BTreeMap<i64, Subject>;
pub type Auditoriums = BTreeMap<i64, Auditorium>;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Group {
    pub name: String,
    pub direction_id: Option<i32>,
    pub speciality_id: Option<i32>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Teacher {
    pub abbr: String,
    pub name: String,
    pub department_id: Option<i32>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Subject {
    pub abbr: String,
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Auditorium {
    pub name: String,
    pub floor: i8,
    pub power: bool,
    pub building: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Timetable {
    pub events: BTreeMap<i64, Event>,
    pub subjects: BTreeMap<i64, EventSubject>,
    pub auditoriums: BTreeMap<i64, EventAuditorium>,
    pub groups: BTreeMap<i64, EventGroup>,
    pub teachers: BTreeMap<i64, EventTeacher>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Event {
    pub starts_at: i64,
    pub ends_at: i64,
    pub kind: EventKind,
    pub count: u8,
    pub subject: i64,
    pub auditorium: i64,
    pub groups: Vec<i64>,
    pub teachers: Vec<i64>,
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventGroup {
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventTeacher {
    pub abbr: String, // TODO: Option? not mandatory, erroneous, can be computed
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventSubject {
    pub abbr: String,
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventAuditorium {
    pub name: String,
}
