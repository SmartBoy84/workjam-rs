use chrono::Local;
use serde::Deserialize;

use super::{
    Employee, Location, Position,
    approval_req::{ApprovalReqParticipant, ApprovalReqStatus, ApprovalReqType},
    events::{self, EventData, Shift},
};

// not using ApprovalReqRes because this doesn't have location, request_details, submission_timestamp
// could wrap in Option<> but would be confusing for user because whether or not the field is present
// is deterministic rather than random
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShiftApprovalReq {
    // ignored displayStatus
    pub id: String,
    #[serde(rename = "type")]
    pub req_type: ApprovalReqType,
    pub status: ApprovalReqStatus,
    pub request_date_time: chrono::NaiveDateTime, // for some reason this doesn't have timezone info

    // sub-structs
    pub initiator: ApprovalReqParticipant,
    pub participants: Vec<ApprovalReqParticipant>,
}

// similar to above not using EventData because shift segement doesn't have ID
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShiftSegment {
    pub start_date_time: chrono::DateTime<Local>,
    pub end_date_time: chrono::DateTime<Local>,
    pub title: Option<String>,
    pub note: Option<String>,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BreakType {
    Meal,
    // othwers?
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShiftBreak {
    #[serde(rename = "type")]
    break_type: BreakType,
    start_date_time: chrono::DateTime<Local>,
    end_date_time: chrono::DateTime<Local>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assignee {
    profile: Employee,
    breaks: Vec<ShiftBreak>, // status, bookingMethod
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShiftRes {
    id: String,
    event: EventData<events::Shift>,
    assignees: Vec<Assignee>,
    position: Position,
    created_by: Employee,
    segments: Vec<ShiftSegment>,
    approval_requests: Vec<ShiftApprovalReq>,
    // status, quantity, offeredSpots, openSpots, allowedActions, externalFields, locked, createViaIntegration
}
