use chrono::Local;
use serde::Deserialize;

use super::{Employee, Location, Position};

#[derive(Deserialize, Debug)]
pub enum ApprovalReqType {
    #[serde(rename = "V5_SHIFT_EDIT")]
    Edit,
    #[serde(rename = "V5_SHIFT_DIRECT_OFFER")]
    Offer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApprovalReqStatus {
    Approved,
    Expired,
    Canceled,
    Pending
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqParticipant {
    profile: Employee,
    status: ApprovalReqStatus,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqDetails {
    start_timestamp: chrono::DateTime<Local>,
    end_timestamp: chrono::DateTime<Local>,
    location: Location,
    position: Position,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReq {
    // ignored displayStatus
    id: String,
    #[serde(rename = "type")]
    req_type: ApprovalReqType,
    status: ApprovalReqStatus,
    request_date_time: chrono::NaiveDateTime, // for some reason this doesn't have timezone info
    submission_timestamp: chrono::DateTime<Local>,
    // sub-structs
    initiator: ApprovalReqParticipant,
    location: Location,
    participants: Vec<ApprovalReqParticipant>,
    request_details: ApprovalReqDetails,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct ApprovalReqsRes(pub Vec<ApprovalReq>);
