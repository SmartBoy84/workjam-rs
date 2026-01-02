use std::marker::PhantomData;

use crate::backend::request::endpoints::ShiftDetail;

use super::{
    super::{
        HasCompanyID, RequestConfig, WorkjamRequest, config::WorkjamRequestConfig,
        endpoints::Coworkers,
    },
    Location,
};

use chrono::Local;
use serde::Deserialize;

pub trait EventType {}
#[derive(Debug)]
pub struct Shift;
#[derive(Debug)]
pub struct Availability;
impl EventType for Shift {}
impl EventType for Availability {}

// Shared fields across all event types
#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Event {
    Shift(EventData<Shift>),
    #[serde(rename = "AVAILABILITY_AVAILABLE")]
    Availability(EventData<Availability>),
    #[serde(other)]
    Unknown, // if I don't know, I don't care
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct EventsRes(pub Vec<Event>);

impl EventData<Shift> {
    pub fn coworkers<C: RequestConfig + HasCompanyID>(&self, c: &C) -> WorkjamRequest<Coworkers> {
        WorkjamRequest::new(
            &WorkjamRequestConfig::new()
                .shift_id(&self.id)
                .location_id(&self.location.id)
                .company_id(c.company_id()),
        )
    }

    pub fn details<C: RequestConfig + HasCompanyID>(&self, c: &C) -> WorkjamRequest<ShiftDetail> {
        WorkjamRequest::new(
            &WorkjamRequestConfig::new()
                .shift_id(&self.id)
                .location_id(&self.location.id)
                .company_id(c.company_id()),
        )
    }
}
