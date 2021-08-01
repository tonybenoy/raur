use crate::aur::{AurPkg, AurResponse};
pub trait Download {
    fn download(&self, dest: String);
}

pub trait Query {
    fn search(&self, query_by: &str, query: &str) -> Result<AurResponse, reqwest::Error>;
    fn info(&self, query: &str) -> Result<AurResponse, reqwest::Error>;
}
