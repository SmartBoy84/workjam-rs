use serde::Deserialize;

#[derive(Deserialize)]
pub struct Position {
    pub id: String,
    pub name: String,
    // externalId, externalCode, sequence ignored
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoworkersGroup {
    pub position: Position,
    pub employees: Vec<Employee>,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct CoworkersRes(pub Vec<CoworkersGroup>);
