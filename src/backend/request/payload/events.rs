use super::Location;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    Shift,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub start_date_time: chrono::DateTime<Local>,
    pub end_date_time: chrono::DateTime<Local>,
    pub title: String,
    pub note: Option<String>,
    pub location: Location,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct EventRes(Vec<Event>);
