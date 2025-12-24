use serde::Deserialize;

pub mod coworkers;
pub mod events;
pub mod notifications;

#[derive(Debug, Deserialize)]
pub struct AuthRes {
    pub userId: u64,
    pub employers: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    id: String,
    name: String,
    #[serde(rename = "type")]
    location_type: LocationType,
    time_zone_id: String,
    // externalCode, externalId ignored
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationType {
    Store,
    #[serde(untagged)]
    Unknown(String),
}
