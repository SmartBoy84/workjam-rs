use serde::Deserialize;

use crate::requests::Position;

use super::Employee;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoworkersGroup {
    pub position: Position,
    pub employees: Vec<Employee>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct CoworkersRes(pub Vec<CoworkersGroup>);
