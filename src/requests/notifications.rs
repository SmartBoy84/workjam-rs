use restman_rs::request::{ApiRequest, RequestConfig};
use serde::{Deserialize, Serialize};

use crate::{
    config::{HasCompanyID, HasEmployeeID, WorkjamRequestConfig},
    endpoints::NotifRead,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum EntityType {
    Shift,
    TimeCard,
    Availability,
    #[serde(untagged)]
    Unknown(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    Info,
    #[serde(untagged)]
    Unknown(String),
}

// ignored: deepLinkDTO, imageURL, refersTo, to (will always be me?)
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub body: String,
    #[serde(rename = "type")]
    pub notif_type: NotificationType,
    pub unix_time_created: u64, // epoch time - can parse into a chrono::DateTime
}

#[derive(Deserialize, Debug)]
pub struct NotifRes {
    pub notifications: Vec<Notification>,
}

impl Notification {
    pub fn set_read<C: RequestConfig + HasCompanyID + HasEmployeeID>(
        &self,
        c: &C,
    ) -> ApiRequest<NotifRead> {
        ApiRequest::new(
            &WorkjamRequestConfig::new()
                .company_id(c.company_id())
                .employee_id(c.employee_id())
                .notification_id(&self.id),
        )
    }
}
