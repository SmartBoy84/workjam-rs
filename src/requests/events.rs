use std::marker::PhantomData;

use chrono::Local;
use restman_rs::request::{ApiRequest, RequestConfig};
use serde::Deserialize;

use crate::{
    config::{HasCompanyID, WorkjamRequestConfig},
    endpoints::{Coworkers, ShiftDetail},
    requests::Location,
};

pub trait EventType {}

#[derive(Debug)]
pub struct Shift;
#[derive(Debug)]
pub struct Availability;

impl EventType for Shift {}
impl EventType for Availability {}

const FMT: &str = "%d/%m/%Y, %a %I:%M%p";

// Shared fields across all event types
#[derive(Deserialize, Debug, derive_more::Display)]
#[display("{} - {}", start_date_time.format(FMT), end_date_time.format(FMT))]
#[serde(rename_all = "camelCase")]
pub struct EventData<T: EventType> {
    pub id: String,
    pub start_date_time: chrono::DateTime<Local>,
    pub end_date_time: chrono::DateTime<Local>,
    pub title: Option<String>,
    pub note: Option<String>,
    pub location: Location,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

#[derive(Deserialize, Debug, derive_more::Display)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    #[display("Shift: {_0}")]
    Shift(EventData<Shift>),

    #[display("Available: {_0}")]
    #[serde(rename = "AVAILABILITY_AVAILABLE")]
    Availability(EventData<Availability>),

    #[display("Unavailable: {_0}")]
    #[serde(rename = "AVAILABILITY_UNAVAILABLE")]
    Unavailability(EventData<Availability>),

    #[display("Time off: {_0}")]
    #[serde(rename = "AVAILABILITY_TIME_OFF")]
    TimeOff(EventData<Availability>),

    #[display("Unknown<{event_type}>")]
    #[serde(untagged)]
    Unknown {
        #[serde(rename = "type")]
        event_type: String,
    }, // if I don't know, I don't care
}

pub type EventsRes = Vec<Event>;

impl EventData<Shift> {
    pub fn employees<C: RequestConfig + HasCompanyID>(&self, c: &C) -> ApiRequest<Coworkers> {
        ApiRequest::new(
            &WorkjamRequestConfig::new()
                .shift_id(&self.id)
                .location_id(&self.location.id)
                .company_id(c.company_id()),
        )
    }

    pub fn details<C: RequestConfig + HasCompanyID>(&self, c: &C) -> ApiRequest<ShiftDetail> {
        ApiRequest::new(
            &WorkjamRequestConfig::new()
                .shift_id(&self.id)
                .location_id(&self.location.id)
                .company_id(c.company_id()),
        )
    }
}
