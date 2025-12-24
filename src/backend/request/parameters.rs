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
