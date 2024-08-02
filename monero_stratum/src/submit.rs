use crate::Request;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Submit {
    id: usize,
    jsonrpc: &'static str,
    method: &'static str,
    params: Parameter,
}

#[derive(Debug, Serialize)]
struct Parameter {
    id: String,
    job_id: String,
    nonce: String,
    result: String,
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
    status: String,
}

impl Submit {
    pub fn new(
        request_id: usize,
        id: String,
        job_id: String,
        nonce: String,
        result: String,
    ) -> Self {
        Self {
            id: request_id,
            jsonrpc: "2.0",
            method: "submit",
            params: Parameter {
                id,
                job_id,
                nonce,
                result,
            },
        }
    }
}

impl Request<Submit, Response> for Submit {}
