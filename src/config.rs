use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    environment: HashMap<String, String>,
    tests: Vec<TestConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestConfig {
    name: String,
    method: String,
    url: String,
    response: Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: Option<String>,
    header: HashMap<String, String>,
    body: Option<String>,
}
