#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serialisation error")]
    Serde { source: serde_json::Error },

    #[error("API error")]
    Request { source: ureq::Error },

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl Error {
    pub fn wrap(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Other(Box::new(err))
    }
}

impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            ureq::Error::Json(source) => Self::Serde { source },
            ureq::Error::Other(source) => Self::Other(source),
            source => Self::Request { source },
        }
    }
}
