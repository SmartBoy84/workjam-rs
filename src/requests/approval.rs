use chrono::Local;
use restman_rs::request::{ApiRequest, RequestConfig};
use serde::Deserialize;

use crate::{
    config::{HasCompanyID, HasEmployeeID, WorkjamRequestConfig},
    endpoints::AcceptApprovalReq,
    requests::{Employee, Location, Position},
};

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
    Pending,
    Retracted
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqParticipant {
    pub profile: Employee,
    pub status: ApprovalReqStatus,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqDetails {
    pub start_timestamp: chrono::DateTime<Local>,
    pub end_timestamp: chrono::DateTime<Local>,
    pub location: Location,
    pub position: Position,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqRes {
    // ignored displayStatus
    pub id: String,
    #[serde(rename = "type")]
    pub req_type: ApprovalReqType,
    pub status: ApprovalReqStatus,
    pub request_date_time: chrono::NaiveDateTime, // for some reason this doesn't have timezone info

    // sub-structs
    pub initiator: ApprovalReqParticipant,
    pub participants: Vec<ApprovalReqParticipant>,

    // some fields don't come up in the approvalRequests field for ShiftDetails
    pub location: Location,
    pub request_details: ApprovalReqDetails,
    pub submission_timestamp: chrono::DateTime<Local>,
}

pub type ApprovalReqsRes = Vec<ApprovalReqRes>;

impl ApprovalReqRes {
    pub fn accept<C: RequestConfig + HasCompanyID + HasEmployeeID>(
        &self,
        c: &C,
    ) -> ApiRequest<AcceptApprovalReq> {
        ApiRequest::new(
            &WorkjamRequestConfig::new()
                .approval_req_id(&self.id)
                .company_id(c.company_id())
                .employee_id(c.employee_id()),
        )
    }
}
