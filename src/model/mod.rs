use serde::Deserialize;
use serde_json::{Serializer, Deserializer};

const METHODS: [&str; 6] = ["get", "post", "patch", "delete", "option", "head"];

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub routes: Vec<Route>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub method: String,
    pub endpoint: String,
    pub responses: Vec<Response>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub body: String,
    pub file_path: String,
    pub status_code: u16,
    pub headers: Vec<Header>
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub key: String,
    pub value: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenClaim {
    cid: String,
    country: String
}

impl TokenClaim {
    pub fn user(&self) -> String {
        format!("{}-{}", self.cid.clone(), self.country.clone())
    }
}