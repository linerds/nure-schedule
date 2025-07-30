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

pub type Id = i64;

pub type Groups = HashSet<Group>;
pub type Teachers = HashSet<Teacher>;
pub type Subjects = HashSet<Subject>;
pub type Auditoriums = HashSet<Auditorium>;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Timetable {
    pub events: HashSet<Event>,
    pub subjects: HashSet<Subject>,
}

#[macro_export]
macro_rules! impl_borrow {
    ($type:ident) => {
        impl PartialEq for $type {
            fn eq(&self, other: &$type) -> bool {
                self.id == other.id
            }
        }

        impl Eq for $type {}

        impl std::hash::Hash for $type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }

        impl ::std::borrow::Borrow<Id> for $type {
            fn borrow(&self) -> &Id {
                &self.id
            }
        }
    };
}
