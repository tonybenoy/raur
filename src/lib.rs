use git2::Repository;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{EnumString, ToString};
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

pub trait Download {
    fn download(&self, dest: String);
}

impl Download for AurPkg {
    fn download(&self, dest: String) {
        println!("Downloading {}...", self.name);
        let repo = match Repository::clone(&self.URL, dest) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }
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
    functionality: Functionality,
    query_by: String,
    query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let _type = args[1].clone();
        let functionality = Functionality::from_str(&_type);
        if functionality.is_err() {
            return Err("functionality not available");
        }
        let functionality = functionality.unwrap();
        functionality.validate(&args[2..args.len()]);

        let query = args[3].clone();
        let query_by = args[2].clone();
        Ok(Config {
            functionality,
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
                ("type", &self.functionality.to_string()),
                ("by", &self.query_by),
                ("arg", &self.query),
            ])
            .send()?;
        let val: AurResponse = serde_json::from_str(&resp.text()?).unwrap();
        Ok(val)
    }
}

#[derive(Debug, EnumString, ToString)]
pub enum Functionality {
    #[strum(serialize = "search", serialize = "s")]
    Search,
    #[strum(serialize = "info", serialize = "i")]
    Info,
    #[strum(disabled)]
    Install,
}

trait Validations {
    fn validate(&self, arguments: &[String]) -> Result<(), Box<dyn Error>>;
}

impl Validations for Functionality {
    fn validate(&self, arguments: &[String]) -> Result<(), Box<dyn Error>> {
        match self {
            Functionality::Search => {
                if arguments.len() < 4 {
                    return Err("not enough arguments".into());
                } else {
                    Ok(())
                }
            }
            Functionality::Info => {
                if arguments.len() < 3 {
                    return Err("not enough arguments".into());
                }
                Ok(())
            }
            Functionality::Install => Ok(()),
        }
    }
}
