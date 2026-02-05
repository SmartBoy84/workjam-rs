use serde::Deserialize;

use crate::requests::Position;

use super::Employee;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoworkersGroup {
    pub position: Position,
    pub employees: Vec<Employee>,
}

pub type CoworkersRes = Vec<CoworkersGroup>;
