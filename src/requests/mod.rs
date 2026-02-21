use serde::Deserialize;

pub mod approval;
pub mod coworkers;
pub mod employee;
pub mod events;
pub mod notifications;
pub mod shift;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthRes {
    pub user_id: u64,
    pub employers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnsiteRes {
    pub on_site: bool, // valid geofences crap ignored
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub location_type: LocationType,
    pub time_zone_id: Option<String>, // externalCode, externalId, timeZoneId ignored
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id && self.location_type == other.location_type
        // to ignore time zone id
    }
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationType {
    Store,
    #[serde(untagged)]
    Unknown(String),
}
