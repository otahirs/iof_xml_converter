pub mod result_list;
use serde_derive::{Deserialize, Serialize};

// common elements

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub name: String,
    pub short_name: Option<String>,
    pub id: Option<String>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Course {
    pub name: Option<String>,
    pub length: Option<i64>,
    pub climb: Option<i64>,
    pub number_of_controls: Option<i64>,
    pub course_control: Option<Vec<CourseControl>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CourseControl {
    pub control: String,
    pub leg_length: Option<f32>,
    #[serde(rename = "@type")]
    pub control_type: ControlType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum ControlType {
    Start,
    Control,
    Finish,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Id {
    #[serde(rename = "@type")]
    pub id_type: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    pub name: Name,
    pub id: Option<Vec<Id>>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    pub family: String,
    pub given: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Organisation {
    pub short_name: Option<String>,
    pub name: Option<String>,
}