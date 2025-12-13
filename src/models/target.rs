// src/models/target.rs
#[derive(Debug, Clone)]
pub struct Target {
    pub ip: String,
    pub port: u16,
    pub service: Option<String>,
    pub os: Option<String>,
    pub domains: Vec<String>,
    pub vulns: Vec<String>,
    pub raw_data: Option<serde_json::Value>,
}