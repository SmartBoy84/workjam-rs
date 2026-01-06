use chrono::Local;
use serde::Deserialize;

use crate::requests::{
    Employee, Location, Position,
    approval::{ApprovalReqParticipant, ApprovalReqStatus, ApprovalReqType},
    events::{EventData, Shift},
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
    #[serde(rename = "type")]
    pub segment_type: ShiftSegmentType,
    pub start_date_time: chrono::DateTime<Local>,
    pub end_date_time: chrono::DateTime<Local>,
    pub position: Position,
    pub location: Location,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BreakType {
    Meal,
    // othwers?
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShiftSegmentType {
    BreakMeal,
    Shift, // othwers?
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
    pub id: String,
    pub event: EventData<Shift>,
    pub assignees: Vec<Assignee>,
    pub position: Position,
    pub created_by: Employee,
    pub segments: Vec<ShiftSegment>,
    pub approval_requests: Vec<ShiftApprovalReq>,
    // status, quantity, offeredSpots, openSpots, allowedActions, externalFields, locked, createViaIntegration
}
