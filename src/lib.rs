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

pub trait Query {
    fn search(&self, query_by: &str, query: &str) -> Result<AurResponse, reqwest::Error>;
    fn info(&self, query: &str) -> Result<AurResponse, reqwest::Error>;
}

pub struct Config {
    _type: String,
    query_by: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let _type = args[1].clone();
        let query = args[2].clone();
        let query_by = args[3].clone();
        Ok(Config {
            _type,
            query,
            query_by,
        })
    }
    pub fn run(&self) -> Result<AurResponse, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get("https://aur.archlinux.org/rpc/")
            .query(&[
                ("v", "5"),
                ("type", &self._type),
                ("by", &self.query_by),
                ("arg", &self.query),
            ])
            .send()?;
        let val_resp = &resp.text()?;
        let val: AurResponse = serde_json::from_str(val_resp).unwrap();
        Ok(val)
    }
}
