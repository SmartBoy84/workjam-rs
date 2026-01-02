use serde::Deserialize;

pub mod coworkers;
pub mod employee;
pub mod events;
pub mod notifications;
pub mod approval_req;

#[derive(Debug, Deserialize)]
pub struct AuthRes {
    pub userId: u64,
    pub employers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsiteRes {
    pub on_site: bool, // valid geofences crap ignored
}

#[derive(Deserialize)]
pub struct WorkingStatusRes {
    #[serde(rename = "employeeWorking")]
    pub is_working: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

impl Employee {
    pub fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Position {
    pub id: String,
    pub name: String,
    // externalId, externalCode, sequence ignored
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: LocationType,
    // externalCode, externalId, timeZoneId ignored
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationType {
    Store,
    #[serde(untagged)]
    Unknown(String),
}
