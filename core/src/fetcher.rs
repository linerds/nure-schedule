pub mod cist;
pub mod mindenit;

use crate::models::{AuditoriumFull, Event, Group, Teacher};

use std::fmt::Display;

pub trait Fetcher {
    fn fetch_groups(&self) -> Result<Vec<Group>, Error>;

    fn fetch_teachers(&self) -> Result<Vec<Teacher>, Error>;

    fn fetch_auditoriums(&self) -> Result<Vec<AuditoriumFull>, Error>;

    fn fetch_events(&self, timetable: Timetable) -> Result<Vec<Event>, Error>;
}

#[derive(Debug)]
pub enum Timetable {
    Group(i64),
    Teacher(i64),
    Auditory(i64),
}

impl Display for Timetable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Group(id) => write!(f, "groups/{id}"),
            Self::Teacher(id) => write!(f, "teachers/{id}"),
            Self::Auditory(id) => write!(f, "auditoriums/{id}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialisation error")]
    UnexpectedFormat { source: serde_json::Error },

    #[error("Request error")]
    Request { source: ureq::Error },

    #[error("API error")]
    Response {
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        resource: String,
        status: u16,
        reason: String,
    },

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Json(source) => Self::UnexpectedFormat { source },
            ureq::Error::Other(source) => Self::Other(source),
            source => Self::Request { source },
        }
    }
}
