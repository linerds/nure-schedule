use crate::{Event, EventAuditorium, EventGroup, EventKind, EventSubject, EventTeacher, Timetable};

use std::collections::BTreeMap;

use serde::{
    Deserialize,
    de::{Deserializer, SeqAccess, Visitor},
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
                // I wish there was a private derife for Default
                let mut timetable = Timetable {
                    events: BTreeMap::default(),
                    subjects: BTreeMap::default(),
                    auditoriums: BTreeMap::default(),
                    groups: BTreeMap::default(),
                    teachers: BTreeMap::default(),
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
                    let mut event_groups = Vec::with_capacity(groups.len());
                    for (id, group) in groups.into_iter().map(Into::into) {
                        event_groups.push(id);
                        timetable.groups.insert(id, group);
                    }

                    let mut event_teachers = Vec::with_capacity(teachers.len());
                    for (id, teacher) in teachers.into_iter().map(Into::into) {
                        event_teachers.push(id);
                        timetable.teachers.insert(id, teacher);
                    }

                    let (subject_id, subject) = subject.into();
                    let (auditorium_id, auditorium) = auditorium.into();

                    timetable.subjects.insert(subject_id, subject);
                    timetable.auditoriums.insert(auditorium_id, auditorium);
                    timetable.events.insert(
                        id,
                        Event {
                            starts_at: started_at,
                            ends_at: ended_at,
                            kind: kind.into(),
                            count,
                            subject: subject_id,
                            auditorium: auditorium_id,
                            groups: event_groups,
                            teachers: event_teachers,
                        },
                    );
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
    /// The sequence number of the current class
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
impl From<EventGroupRaw> for (i64, EventGroup) {
    fn from(EventGroupRaw { id, name }: EventGroupRaw) -> Self {
        (id, EventGroup { name })
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventTeacherRaw {
    pub id: i64,
    pub full_name: String,
    pub short_name: String,
}
impl From<EventTeacherRaw> for (i64, EventTeacher) {
    fn from(
        EventTeacherRaw {
            id,
            full_name: name,
            short_name: abbr,
        }: EventTeacherRaw,
    ) -> Self {
        (id, EventTeacher { abbr, name })
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventSubjectRaw {
    pub id: i64,
    pub title: String,
    pub brief: String,
}
impl From<EventSubjectRaw> for (i64, EventSubject) {
    fn from(
        EventSubjectRaw {
            id,
            title: name,
            brief: abbr,
        }: EventSubjectRaw,
    ) -> Self {
        (id, EventSubject { abbr, name })
    }
}

#[derive(Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EventAuditoriumRaw {
    pub id: i64,
    pub name: String,
}
impl From<EventAuditoriumRaw> for (i64, EventAuditorium) {
    fn from(EventAuditoriumRaw { id, name }: EventAuditoriumRaw) -> Self {
        (id, EventAuditorium { name })
    }
}
