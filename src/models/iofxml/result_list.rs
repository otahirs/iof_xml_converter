use crate::models::iofxml::{Class, Course, Event, Organisation, Person};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ResultList {
    #[serde(rename = "@createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "@creator")]
    pub creator: Option<String>,
    #[serde(rename = "@iofVersion")]
    pub iof_version: Option<String>,
    #[serde(rename = "@status")]
    pub status: Option<String>,
    pub event: Event,
    pub class_result: Vec<ClassResult>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClassResult {
    pub class: Class,
    pub course: Option<Course>,
    #[serde(default)]
    pub person_result: Vec<PersonResult>,
    #[serde(default)]
    pub team_result: Vec<TeamResult>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PersonResult {
    pub entry_id: Option<String>,
    pub person: Person,
    pub organisation: Option<Organisation>,
    pub result: Result,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TeamResult {
    pub entry_id: Option<String>,
    pub bib_number: Option<String>,
    pub name: String,
    pub organisation: Option<Vec<Organisation>>,
    #[serde(default)]
    pub team_member_result: Vec<TeamMemberResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TeamMemberResult {
    pub person: Option<Person>,
    pub result: Result,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Result {
    pub leg: Option<i16>,
    pub bib_number: Option<String>,
    pub start_time: Option<String>,
    pub finish_time: Option<String>,
    pub time: Option<f32>,
    pub time_behind: Option<f32>,
    pub position: Option<i32>,
    pub status: Option<String>,
    pub split_time: Option<Vec<SplitTime>>,
    pub control_card: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SplitTime {
    pub control_code: i16,
    pub time: Option<f32>,
}