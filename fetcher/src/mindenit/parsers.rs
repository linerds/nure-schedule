mod auditorium;
mod group;
mod subject;
mod teacher;
mod timetable;

use auditorium::AuditoriumRaw;
use group::GroupRaw;
use subject::SubjectRaw;
use teacher::TeacherRaw;
use timetable::TimetableParser;

use crate::{ArrayToMap, Auditorium, Group, Subject, Teacher, Timetable};

use std::collections::HashSet;

#[derive(serde::Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct Health {
    uptime: f64,
    message: String,
    date: String,
}

pub type ResponseTimetable = Response<TimetableParser>;
pub type ResponseGroups = Response<ArrayToMap<GroupRaw, Group>>;
pub type ResponseTeachers = Response<ArrayToMap<TeacherRaw, Teacher>>;
pub type ResponseSubjects = Response<ArrayToMap<SubjectRaw, Subject>>;
pub type ResponseAuditoriums = Response<ArrayToMap<AuditoriumRaw, Auditorium>>;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // HACK: `success` is useless, remove it?
pub struct Response<T> {
    data: Option<T>,
    success: Option<bool>,
    error: Option<String>,
    message: Option<String>,
    status_code: Option<u16>,
}

impl<T> Response<T> {
    fn data<V>(self) -> Result<V, Box<dyn std::error::Error>>
    where
        T: Into<V>,
    {
        if let Some(data) = self.data {
            return Ok(data.into());
        }

        // more descriptive than error
        if let Some(message) = self.message {
            return Err(message.into());
        }
        if let Some(error) = self.error {
            return Err(error.into());
        }
        // NOTE: message and error are capitalized, following the style
        if let Some(code) = self.status_code {
            return Err(format!("Status code {code}").into());
        }

        Err("No relevant information".into())
    }
}

impl TryFrom<ResponseTimetable> for Timetable {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: ResponseTimetable) -> Result<Self, Self::Error> {
        value.data()
    }
}

impl<R, T> TryFrom<Response<ArrayToMap<R, T>>> for HashSet<T> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Response<ArrayToMap<R, T>>) -> Result<Self, Self::Error> {
        value.data()
    }
}

// just works tbh
// impl<T, V> From<Response<Vec<T>>> for BTreeMap<i64, V>
// where
//     T: Into<(i64, V)>,
// {
//     fn from(value: Response<Vec<T>>) -> Self {
//         value.data.into_iter().map(Into::into).collect()
//     }
// }

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    use crate::ResponseError;

    use serde::Deserialize;

    #[test]
    fn health() -> Result<(), serde_json::Error> {
        let health: Health = serde_json::from_str(include_str!("../../test-data/health.json"))?;
        println!("{health:#?}");
        Ok(())
    }

    fn parse_print_first<'de, Raw, Val>(data: &'de str) -> Result<(), ResponseError>
    where
        Raw: Deserialize<'de> + Into<Val>,
        Val: std::fmt::Debug + std::hash::Hash + Eq,
    {
        let response: Response<ArrayToMap<Raw, Val>> = serde_json::from_str(data)?;
        println!(
            "{:#?}",
            HashSet::try_from(response)?
                .iter()
                .next()
                .ok_or_else(|| Box::from("The data is empty"))?
        );
        Ok(())
    }

    #[test]
    fn teachers() -> Result<(), ResponseError> {
        parse_print_first::<TeacherRaw, Teacher>(include_str!("../../test-data/teachers.json"))
    }
    #[test]
    fn auditoriums() -> Result<(), ResponseError> {
        parse_print_first::<AuditoriumRaw, Auditorium>(include_str!(
            "../../test-data/auditoriums.json"
        ))
    }
    #[test]
    fn groups() -> Result<(), ResponseError> {
        parse_print_first::<GroupRaw, Group>(include_str!("../../test-data/groups.json"))
    }
    #[test]
    fn group_teachers() -> Result<(), ResponseError> {
        parse_print_first::<TeacherRaw, Teacher>(include_str!(
            "../../test-data/group-teachers.json"
        ))
    }
    #[test]
    fn group_subjects() -> Result<(), ResponseError> {
        parse_print_first::<SubjectRaw, Subject>(include_str!(
            "../../test-data/group-subjects.json"
        ))
    }

    #[test]
    fn group_timetable() -> Result<(), ResponseError> {
        timetable(include_str!("../../test-data/group-schedule.json"))
    }
    #[test]
    fn teacher_timetable() -> Result<(), ResponseError> {
        timetable(include_str!("../../test-data/teacher-schedule.json"))
    }
    #[test]
    #[ignore = "beefy data"]
    fn auditorium_timetable() -> Result<(), ResponseError> {
        timetable(include_str!("../../test-data/auditorium-schedule.json"))
    }
    // #[test]
    // #[ignore = "beefy data, writes to disk"]
    // fn auditorium_timetable_to_disk() -> Result<(), serde_json::Error> {
    //     use std::fs::File;
    //     use std::io::Write;
    //     let mut file = File::create("/tmp/timetable-parsed-data.txt").unwrap();

    //     let response: ResponseTimetable =
    //         serde_json::from_str(include_str!("../../test-data/auditorium-schedule.json"))?;
    //     let timetable: Timetable = response.into();

    //     writeln!(file, "{timetable:?}").unwrap();
    //     Ok(())
    // }

    fn timetable(data: &str) -> Result<(), ResponseError> {
        let response: ResponseTimetable = serde_json::from_str(data)?;

        let Timetable { events, subjects }: Timetable = response.try_into()?;

        let event = events.iter().next().unwrap();

        let crate::Event {
            id,
            subject,
            auditorium,
            groups: event_groups,
            teachers: event_teachers,
            ..
        } = event;

        let subject = subjects.get(subject).unwrap();

        println!("First event: {id}");
        println!("Subject: {subject:#?}");
        println!("Auditorium: {auditorium}");
        println!("Groups: {event_groups:#?}");
        println!("Teachers: {event_teachers:#?}");

        Ok(())
    }
}
