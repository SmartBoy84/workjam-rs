use serde::Deserialize;

pub mod notifications;

#[derive(Debug, Deserialize)]
pub struct AuthRes {
    pub userId: u64,
    pub employers: Vec<String>,
}
