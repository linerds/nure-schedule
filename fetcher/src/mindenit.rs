mod parsers;

use parsers::{
    ResponseAuditoriums, ResponseGroups, ResponseSubjects, ResponseTeachers, ResponseTimetable,
};

use crate::{
    mindenit::parsers::Health, Auditoriums, Fetcher, FetcherAgent, FetcherError, FetcherExt,
    Groups, Subjects, Teachers, Timetable, TimetableKind,
};

#[derive(Clone, Debug)]
pub struct Mindenit {
    agent: FetcherAgent,
    pub base_url: String,
}

impl Default for Mindenit {
    fn default() -> Self {
        Self::new(FetcherAgent::default())
    }
}

impl Mindenit {
    #![allow(dead_code)] // HACK: useless function?
    fn fetch_health(&self) -> Result<Health, FetcherError> {
        serde_json::from_reader(self.agent.request(&format!("{}/health", self.base_url))?)
            .map_err(Into::into)
    }

    fn fetch<R, T>(&self, endpoint: impl AsRef<str>) -> Result<T, FetcherError>
    where
        R: serde::de::DeserializeOwned + TryInto<T>,
        FetcherError: From<<R as TryInto<T>>::Error>,
    {
        Ok(serde_json::from_reader::<_, R>(self.agent.request(&format!(
            "{}/{}",
            self.base_url,
            endpoint.as_ref()
        ))?)?
        .try_into()?)
    }
}

impl Fetcher for Mindenit {
    fn new(agent: FetcherAgent) -> Self {
        Self {
            agent,
            base_url: "https://sh.mindenit.org/api".into(),
        }
    }
    fn fetch_groups(&self) -> Result<Groups, FetcherError> {
        self.fetch::<ResponseGroups, _>("groups")
    }
    fn fetch_teachers(&self) -> Result<Teachers, FetcherError> {
        self.fetch::<ResponseTeachers, _>("teachers")
    }
    fn fetch_auditoriums(&self) -> Result<Auditoriums, FetcherError> {
        self.fetch::<ResponseAuditoriums, _>("auditoriums")
    }
    fn fetch_timetable(&self, kind: TimetableKind) -> Result<Timetable, FetcherError> {
        self.fetch::<ResponseTimetable, _>(match kind {
            TimetableKind::Group(id) => format!("groups/{id}/schedule"),
            TimetableKind::Teacher(id) => format!("teachers/{id}/schedule"),
            TimetableKind::Auditorium(id) => format!("auditoriums/{id}/schedule"),
        })
    }
}

impl FetcherExt for Mindenit {
    fn fetch_teachers_by_group(&self, id: i64) -> Result<Teachers, FetcherError> {
        self.fetch::<ResponseTeachers, _>(format!("groups/{id}/teachers"))
    }
    fn fetch_subjects_by_group(&self, id: i64) -> Result<Subjects, FetcherError> {
        self.fetch::<ResponseSubjects, _>(format!("groups/{id}/subjects"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::LazyLock;

    static MINDENIT: LazyLock<Mindenit> = LazyLock::new(Mindenit::default);

    #[test]
    #[ignore = "Errors Mindenit"]
    fn fetch_bad_request() {
        let response = MINDENIT.clone().fetch_teachers_by_group(12345678912345);
        println!("{response:#?}");
        assert!(matches!(response, Err(FetcherError::Request(_))));
    }

    // TODO: Request an API for that stuff 🐢
    // #[test]
    // #[ignore = "makes 300 requests... That's crazy 🐢 That's actually crazy 🐢 That's messed up 🐢"]
    // fn find_the_damn_auditorium() -> Result<(), FetcherError> {
    //     for (id, _) in MINDENIT.fetch_auditoriums()? {
    //         let schedule = MINDENIT.fetch_timetable(TimetableKind::Auditorium(id))?;

    //         if !schedule.events.is_empty() {
    //             println!("Auditorium {id} has events!");
    //         }
    //     }
    //     Ok(())
    // }

    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_group() -> Result<(), FetcherError> {
        println!("{:#?}", MINDENIT.clone().fetch_health()?);
        Ok(())
    }
    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_groups() -> Result<(), FetcherError> {
        println!(
            "{:#?}",
            MINDENIT.clone().fetch_groups()?.iter().next().unwrap()
        );
        Ok(())
    }

    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_teachers() -> Result<(), FetcherError> {
        println!(
            "{:#?}",
            MINDENIT.clone().fetch_teachers()?.iter().next().unwrap()
        );
        Ok(())
    }

    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_auditoriums() -> Result<(), FetcherError> {
        println!(
            "{:#?}",
            MINDENIT.clone().fetch_auditoriums()?.iter().next().unwrap()
        );
        Ok(())
    }

    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_group_timetable() -> Result<(), FetcherError> {
        timetable(
            MINDENIT
                .clone()
                .fetch_timetable(TimetableKind::Group(11103296))?,
        );
        Ok(())
    }
    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_teacher_timetable() -> Result<(), FetcherError> {
        timetable(
            MINDENIT
                .clone()
                .fetch_timetable(TimetableKind::Teacher(2145721))?,
        );
        Ok(())
    }
    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_auditorium_timetable() -> Result<(), FetcherError> {
        // With shedules: -4, -2, 17, 26, 67, 93, 94, 95, 119, 172, 1675428,
        // 7693999, 8324566, 11426456, 11426457, 11426458, 11426459, 11616156,
        timetable(
            MINDENIT
                .clone()
                .fetch_timetable(TimetableKind::Auditorium(-4))?,
        ); // DL_1
        Ok(())
    }
    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_teachers_by_group() -> Result<(), FetcherError> {
        println!(
            "{:#?}",
            MINDENIT
                .clone()
                .fetch_teachers_by_group(11103296)?
                .iter()
                .next()
                .unwrap()
        );
        Ok(())
    }
    #[test]
    #[ignore = "Downloads from Mindenit"]
    fn fetch_subjects_by_group() -> Result<(), FetcherError> {
        println!(
            "{:#?}",
            MINDENIT
                .clone()
                .fetch_subjects_by_group(11103296)?
                .iter()
                .next()
                .unwrap()
        );
        Ok(())
    }

    // NOTE: Taken from parsers.rs
    fn timetable(Timetable { events, subjects }: Timetable) {
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
    }
}
