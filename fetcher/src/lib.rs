mod array_to_map;
mod error;
mod fetcher_agent;

use array_to_map::ArrayToMap;
use fetcher_agent::FetcherAgent;

pub use error::*;

mod mindenit;
pub use mindenit::Mindenit;

use schedule_model::*;

pub trait Fetcher: Default {
    fn new(agent: FetcherAgent) -> Self;
    fn fetch_groups(&self) -> Result<Groups, FetcherError>;
    fn fetch_teachers(&self) -> Result<Teachers, FetcherError>;
    fn fetch_auditoriums(&self) -> Result<Auditoriums, FetcherError>;
    fn fetch_timetable(&self, kind: TimetableKind) -> Result<Timetable, FetcherError>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TimetableKind {
    Group(i64),
    Teacher(i64),
    Auditorium(i64),
}

pub trait FetcherExt: Fetcher {
    fn fetch_teachers_by_group(&self, id: i64) -> Result<Teachers, FetcherError>;
    fn fetch_subjects_by_group(&self, id: i64) -> Result<Subjects, FetcherError>;
}
