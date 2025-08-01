mod auditorium;
mod event;
mod group;
mod subject;
mod teacher;

pub use auditorium::Auditorium;
pub use event::{Event, EventKind};
pub use group::Group;
pub use subject::Subject;
pub use teacher::Teacher;

use std::collections::HashSet;

pub type Groups = HashSet<Group>;
pub type Teachers = HashSet<Teacher>;
pub type Subjects = HashSet<Subject>;
pub type Auditoriums = HashSet<Auditorium>;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Timetable {
    pub events: HashSet<Event>,
    pub subjects: HashSet<Subject>,
}
