use crate::fetcher::{Error, Timetable};
use crate::models::{AuditoriumFull, Event, Group, Teacher};

use chrono::{DateTime, Utc};
use serde::{Deserialize, de::DeserializeOwned};

// TODO: that should probably not be hardcoded
const BASE_URL: &str = "https://sh.mindenit.org/api";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Response<T> {
    success: bool,
    data: Vec<T>,
    error: Option<String>,
    message: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Health {
    uptime: f64,
    message: String,
    date: DateTime<Utc>,
}

#[derive(Debug)]
pub struct MindenitFetcher {
    agent: ureq::Agent,
}

impl MindenitFetcher {
    fn fetch<T: DeserializeOwned>(&self, resource: &str) -> Result<T, Error> {
        self.agent
            .get(format!("{BASE_URL}/{resource}"))
            .call()?
            .body_mut()
            .read_json::<T>()
            .map_err(Error::from)
    }

    fn fetch_response<T: DeserializeOwned>(&self, resource: &str) -> Result<Vec<T>, Error> {
        self.fetch::<Response<T>>(resource)
            .map(|response| response.data)
    }

    pub fn fetch_health(&self) -> Result<Health, Error> {
        self.fetch::<Health>("health")
    }
}

impl Default for MindenitFetcher {
    fn default() -> Self {
        let agent = ureq::agent();

        Self { agent }
    }
}

impl super::Fetcher for MindenitFetcher {
    fn fetch_groups(&self) -> Result<Vec<Group>, Error> {
        self.fetch_response("groups")
    }

    fn fetch_teachers(&self) -> Result<Vec<Teacher>, Error> {
        self.fetch_response("teachers")
    }

    fn fetch_auditoriums(&self) -> Result<Vec<AuditoriumFull>, Error> {
        self.fetch_response("auditoriums")
    }

    fn fetch_events(&self, tb: Timetable) -> Result<Vec<Event>, Error> {
        self.fetch_response(&format!("{tb}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetcher::Fetcher;

    // cargo test -- --nocapture
    // cargo test -- --show-output
    #[test]
    fn health_fetch() -> Result<(), Error> {
        let fetcher = MindenitFetcher::default();
        let health = fetcher.fetch_health()?;

        println!("Health Status:");
        println!("\tUptime: {:.2} seconds", health.uptime);
        println!("\tMessage: {}", health.message);
        println!("\tDate: {}", health.date);

        Ok(())
    }

    #[test]
    fn groups_fetch() -> Result<(), Error> {
        let fetcher = MindenitFetcher::default();
        let groups = fetcher.fetch_groups()?;

        println!("Fetched {} groups", groups.len());

        Ok(())
    }

    #[test]
    fn teachers_fetch() -> Result<(), Error> {
        let fetcher = MindenitFetcher::default();
        let teachers = fetcher.fetch_teachers()?;

        println!("Fetched {} teachers", teachers.len());

        Ok(())
    }

    #[test]
    fn auditoriums_fetch() -> Result<(), Error> {
        let fetcher = MindenitFetcher::default();
        let auditoriums = fetcher.fetch_auditoriums()?;

        println!("Fetched {} auditoriums", auditoriums.len());

        Ok(())
    }
}
