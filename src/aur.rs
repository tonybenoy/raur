use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[allow(non_snake_case)]
pub struct AurPkg {
    name: String,
    ID: i32,
    package_base: String,
    PackageBaseID: i32,
    maintainer: String,
    version: String,
    description: String,
    URL: String,
    num_votes: i32,
    popularity: f32,
    out_of_date: Option<bool>,
    LastModified: i64,
    FirstSubmitted: i64,
    depends: Option<Vec<String>>,
    MakeDepends: Option<Vec<String>>,
    OptDepends: Option<Vec<String>>,
    License: Option<String>,
    keywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct AurResponse {
    version: i32,
    r#type: String,
    resultcount: i32,
    results: Vec<AurPkg>,
}
