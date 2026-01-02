use bon::Builder;
use chrono::{DateTime, Local};
use serde::Serialize;

pub trait QueryParameters: Serialize {
    fn add_str(&self, s: &mut String) {
        unsafe {
            s.push('?');
            // WOWZERS! Alright, serde_url_params can't ever fail because I vet my structs before using the unchecked unwrap
            // In addition, Serde will always yield utf8 so I can write directly to the string's underlying buffer
            serde_url_params::to_writer(s.as_mut_vec(), self).unwrap();
            // .unwrap_unchecked();
        }
    }
}

#[derive(Serialize, Builder)]
pub struct NotifPara {
    offset: u32,
    size: u32,
}

impl QueryParameters for NotifPara {}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct EventsPara {
    #[builder(default = true)]
    include_overlaps: bool,
    end_date_time: DateTime<Local>,
    start_date_time: DateTime<Local>,
}

impl QueryParameters for EventsPara {}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmployeeStatus {
    Pending,
    Active,
    Inactive,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EmployeeDetailsField {
    CurrentEmployments,
    PastEmployments,
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct EmployeesDetailsPara {
    // sensible, consistent + what the backend uses when this field is absent
    size: u32,
    statuses: Option<Vec<EmployeeStatus>>,
    extra_fields: Option<Vec<EmployeeDetailsField>>,
    employee_ids: Option<Vec<String>>,
}

impl QueryParameters for EmployeesDetailsPara {}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApprovalReqCatagory {
    MyRequests,
    Archived,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SortBy {
    RequestDateTime,
    // probably others but eh
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SortOrder {
    Descending,
    Ascending,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortPara {
    sort_by: SortBy,
    sort_order: SortOrder,
}

#[derive(Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ApprovalReqPara {
    category: ApprovalReqCatagory,
    #[serde(flatten)]
    sort_para: Option<SortPara>,
}

impl QueryParameters for ApprovalReqPara {}