use chrono::NaiveDate;
use serde::Deserialize;

use super::{Employee, Location, Position};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Employment {
    pub position: Position,
    pub location: Location,
    pub primary: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeDetailsRes {
    #[serde(flatten)]
    pub employee: Employee,
    pub current_employments: Option<Vec<Employment>>,
    pub past_employments: Option<Vec<Employment>>,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct EmployeesDetailsRes(pub Vec<EmployeeDetailsRes>);
