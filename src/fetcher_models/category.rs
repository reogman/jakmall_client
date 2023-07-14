use serde::{Deserialize, Serialize};

pub type JakmallCategories = Vec<Parent>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    pub name: String,
    pub url: String,
    pub icon: String,
    pub color: Option<String>,
    pub children: Vec<Children>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub name: String,
    pub url: String,
    pub children: Vec<Children2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children2 {
    pub name: String,
    pub url: String,
}
