mod error;
mod models;

use error::Error;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Health {
    uptime: f64,
    message: String,
    date: DateTime<Utc>,
}

impl Health {
    pub fn fetch() -> Result<Self, Error> {
        ureq::get("https://sh.mindenit.org/api/health")
            .call()?
            .body_mut()
            .read_json::<Self>()
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test -- --nocapture
    #[test]
    fn health_fetch() -> Result<(), Error> {
        let health = Health::fetch()?;

        println!("Health Status:");
        println!("\tUptime: {:.2} seconds", health.uptime);
        println!("\tMessage: {}", health.message);
        println!("\tDate: {}", health.date);

        Ok(())
    }
}
