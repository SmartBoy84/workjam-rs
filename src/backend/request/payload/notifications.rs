use serde::{Deserialize, Serialize};

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
pub struct Notification {
    pub id: String,
    pub body: String,
    #[serde(rename = "type")]
    pub notif_type: NotificationType,
    #[serde(rename = "unixTimeCreated")]
    pub unix_time_created: u64, // epoch time - can parse into a chrono::DateTime
}

#[derive(Deserialize)]
pub struct NotifRes {
    pub notifications: Vec<Notification>,
}
