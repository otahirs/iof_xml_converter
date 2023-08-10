pub mod result_list;
use serde_derive::{Deserialize, Serialize};

// common elements

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    pub id: Option<Vec<Id>>,
    pub name: String,
    pub start_time: Option<StartTime>,
    pub official: Option<Vec<Official>>,
    pub race: Option<Race>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Race {
    pub race_number: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Official {
    #[serde(rename = "@type")]
    pub official_type: Option<String>,
    pub person: Option<Person>,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StartTime {
    pub date: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Class {
    pub id: Option<String>,
    pub name: String,
    pub short_name: Option<String>,
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
    pub id: Option<Vec<Id>>,
    pub name: Name,
    pub nationality: Option<Nationality>
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
    pub id: Option<Vec<Id>>,
    pub name: Option<String>,
    pub short_name: Option<String>,
    pub country: Option<Country>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    #[serde(rename = "@code")]
    pub code: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Nationality {
    #[serde(rename = "@code")]
    pub code: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}