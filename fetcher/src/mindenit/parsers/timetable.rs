use crate::{Event, Timetable};

use std::collections::HashSet;

use schedule_model::{EventKind, Id, Subject};
use serde::{
    de::{Deserializer, SeqAccess, Visitor},
    Deserialize,
};

pub struct TimetableParser(Timetable);
impl From<TimetableParser> for Timetable {
    fn from(value: TimetableParser) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for TimetableParser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimetableVisitor;

        impl<'de> Visitor<'de> for TimetableVisitor {
            type Value = TimetableParser;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an array of timetable events")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<TimetableParser, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut timetable = Timetable {
                    events: HashSet::default(),
                    subjects: HashSet::default(),
                };

                while let Some(EventRaw {
                    id,
                    started_at,
                    ended_at,
                    count,
                    kind,
                    groups,
                    teachers,
                    subject,
                    auditorium,
                }) = seq.next_element()?
                {
                    let event_groups: HashSet<Id> = groups.iter().map(|g| g.id).collect();
                    let event_teachers: HashSet<Id> = teachers.iter().map(|t| t.id).collect();

                    timetable.events.insert(Event {
                        id,
                        starts_at: started_at,
                        ends_at: ended_at,
                        kind: kind.into(),
                        count,
                        subject: subject.id,
                        auditorium: auditorium.id,
                        groups: event_groups,
                        teachers: event_teachers,
                    });

                    timetable.subjects.insert(subject.into());
                }

                Ok(TimetableParser(timetable))
            }
        }

        deserializer.deserialize_seq(TimetableVisitor)
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventRaw {
    pub id: i64,
    pub started_at: i64,
    pub ended_at: i64,
    /// The sequence number of the event
    #[serde(rename = "numberPair")]
    pub count: u8,

    #[serde(rename = "type")]
    pub kind: EventKindRaw,
    pub groups: Vec<EventGroupRaw>,
    pub teachers: Vec<EventTeacherRaw>,
    pub subject: EventSubjectRaw,
    pub auditorium: EventAuditoriumRaw,
}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum EventKindRaw {
    #[serde(rename = "Лк")]
    Lecture,
    #[serde(rename = "Пз")]
    PracticalWork,
    #[serde(rename = "Лб")]
    LaboratoryWork,
    #[serde(rename = "Конс")]
    Consultation,
    #[serde(rename = "Зал")]
    FinalTest,
    #[serde(rename = "Екз")]
    Exam,
    #[serde(rename = "КП/КР")]
    CourseWork,
    #[serde(other)]
    Unknown,
}
impl From<EventKindRaw> for EventKind {
    fn from(value: EventKindRaw) -> Self {
        match value {
            EventKindRaw::Lecture => Self::Lecture,
            EventKindRaw::PracticalWork => Self::PracticalWork,
            EventKindRaw::LaboratoryWork => Self::LaboratoryWork,
            EventKindRaw::Consultation => Self::Consultation,
            EventKindRaw::FinalTest => Self::FinalTest,
            EventKindRaw::Exam => Self::Exam,
            EventKindRaw::CourseWork => Self::CourseWork,
            EventKindRaw::Unknown => Self::Unknown,
        }
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventGroupRaw {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventTeacherRaw {
    pub id: i64,
    pub full_name: String,
    pub short_name: String,
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventSubjectRaw {
    pub id: i64,
    pub title: String,
    pub brief: String,
}
impl From<EventSubjectRaw> for Subject {
    fn from(
        EventSubjectRaw {
            id,
            title: name,
            brief: abbr,
        }: EventSubjectRaw,
    ) -> Self {
        Self { id, abbr, name }
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventAuditoriumRaw {
    pub id: i64,
    pub name: String,
}
