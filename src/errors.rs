use std::error::Error;
use tokio_walltime;

use chrono;

#[derive(Debug)]
pub enum HeliocronError {
    Config(ConfigErrorKind),
    Runtime(RuntimeErrorKind),
}

#[derive(Debug, Clone)]
pub enum ConfigErrorKind {
    InvalidCoordindates(&'static str),
    InvalidTomlFile,
    ParseDate,
    ParseAltitude,
    ParseOffset,
    InvalidEvent,
}

impl ConfigErrorKind {
    fn as_str(&self) -> &str {
        match *self {
            ConfigErrorKind::InvalidCoordindates(msg) => msg,
            ConfigErrorKind::InvalidTomlFile => {
                "Error parsing TOML file. Ensure that it is of the correct format."
            }
            ConfigErrorKind::ParseDate => {
                "Error parsing date. Ensure the date and timezone formats are correct."
            }
            ConfigErrorKind::ParseAltitude => {
                "Error parsing altitude. Must be a number which is <= 90.0 and >= -90.0."
            }
            ConfigErrorKind::ParseOffset => {
                "Error parsing offset. Expected a string in the format HH:MM:SS or HH:MM."
            }
            ConfigErrorKind::InvalidEvent => "Error parsing event.",
        }
    }
}

#[derive(Debug)]
pub enum RuntimeErrorKind {
    NonOccurringEvent,
    PastEvent,
    EventMissed(i64),
    SleepError(tokio_walltime::Error),
}

impl std::fmt::Display for HeliocronError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Config(ref err) => write!(
                f,
                "Config error: {}",
                match err {
                    ConfigErrorKind::InvalidCoordindates(msg) =>
                        format!("Invalid coordinates - {}", msg),
                    ConfigErrorKind::InvalidTomlFile => err.as_str().to_string(),
                    ConfigErrorKind::ParseDate => err.as_str().to_string(),
                    ConfigErrorKind::ParseAltitude => err.as_str().to_string(),
                    ConfigErrorKind::ParseOffset => err.as_str().to_string(),
                    ConfigErrorKind::InvalidEvent => err.as_str().to_string(),
                }
            ),
            Self::Runtime(ref err) => write!(
                f,
                "Runtime error: {}",
                match err {
                    RuntimeErrorKind::NonOccurringEvent =>
                        "The chosen event does not occur on this day.".to_string(),
                    RuntimeErrorKind::PastEvent => {
                        "The chosen event occurred in the past; cannot wait a negative amount of time.".to_string()
                    }
                    RuntimeErrorKind::EventMissed(by) => format!("Event missed by {by}s"),
                    RuntimeErrorKind::SleepError(e) => e.to_string(),
                }
            ),
        }
    }
}

impl Error for HeliocronError {}

impl From<chrono::ParseError> for HeliocronError {
    fn from(_err: chrono::ParseError) -> Self {
        HeliocronError::Config(ConfigErrorKind::ParseDate)
    }
}

impl From<tokio_walltime::Error> for HeliocronError {
    fn from(err: tokio_walltime::Error) -> Self {
        HeliocronError::Runtime(RuntimeErrorKind::SleepError(err))
    }
}
