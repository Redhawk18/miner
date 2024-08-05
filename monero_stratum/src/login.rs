use crate::Request;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Login {
    id: usize,
    jsonrpc: &'static str,
    method: &'static str,
    params: Parameter,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parameter {
    login: String,
    pass: String,
    agent: &'static str,
    rigid: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    id: usize,
    jsonrpc: String,
    error: Value,
    result: Result,
}

#[derive(Debug, Deserialize)]
struct Result {
    id: String,
    job: Job,
    status: String,
}

#[derive(Debug, Deserialize)]
struct Job {
    blob: String,
    height: i64,
    job_id: String,
    seed_hash: String,
    target: String,
}

impl Login {
    pub fn new(id: usize, login: String, pass: String) -> Self {
        Self {
            id,
            jsonrpc: "2.0",
            method: "login",
            params: Parameter {
                login,
                pass,
                agent: concat!(env!("CARGO_CRATE_NAME"), "/", env!("CARGO_PKG_VERSION")),
                rigid: "mythra testing".to_string(),
            },
        }
    }
}

impl Request<Login, Response> for Login {}
