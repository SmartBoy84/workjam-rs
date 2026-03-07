use restman_rs::request::QueryPayload;
use serde::Serialize;

pub type AvailabilityRes = (); // todo

// TODO; implement custom serialisation
#[derive(Serialize)]
pub struct AvailabilityDuration {
    days: usize,
    hrs: usize,
    min: usize,
    secs: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AvailabilityType {
    Available,
    Unavailable,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    #[serde(rename = "type")]
    availability_type: AvailabilityType,
    day_index: usize,
    duration: AvailabilityDuration,
}



#[derive(Serialize)]
pub struct AvailabilityPayload {}

impl QueryPayload for AvailabilityPayload {}
