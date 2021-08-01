use crate::aur::{AurPkg, AurResponse};
use std::error::Error;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::{EnumString, ToString};

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
