use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    sync::Arc,
};

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Serialize;

struct State {
    time_frame: TimeFrame,
    events: BTreeMap<DateTime<Tz>, Vec<Event>>,
}

// Profile of settings
#[derive(Serialize)]
struct Schedule {
    name: String,
    filters: Vec<Filter>,
}

#[derive(Default)]
enum TimeFrame {
    Day,
    #[default]
    Week,
    Month,
}

#[derive(Serialize, Clone, Default)]
struct Filter {
    exclude: bool, // include is default
    groups: BTreeSet<i64>,
    teachers: BTreeSet<i64>,
    auditoriums: BTreeSet<i64>,
    kinds: BTreeSet<EventKind>,
}

struct Event {
    id: i64,
    count: u8,
    kind: EventKind,
    starts_at: DateTime<Tz>,
    // ends_at: DateTime<Tz>,
    subject: Arc<Subject>,
    auditorium: Arc<Auditorium>,
    groups: Vec<Arc<Group>>,
    teachers: Vec<Arc<Teacher>>,
}

#[derive(Serialize, Clone, Copy)]
enum EventKind {
    Lecture,
    PracticalWork,
    LaboratoryWork,
    Consultation,
    FinalTest, // Залік
    Exam,
    CourseWork,
}
struct Group {
    id: i64,
    name: String,
}
struct Teacher {
    id: i64,
    full_name: String,
    short_name: String,
}
struct Subject {
    id: i64,
    abbr: String,
    name: String,
}
struct Auditorium {
    id: i64,
    name: String,
    floor: i8,
    has_power: bool,
    building: String,
}
