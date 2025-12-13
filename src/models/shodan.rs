// src/models/shodan.rs
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AccountProfile {
    pub member: bool,
    pub display_name: Option<String>,
    pub credits: u64,
    pub username: Option<String>,
    pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanMeta {
    pub module: Option<String>,
    pub crawler: Option<String>,
    pub id: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanLocation {
    pub city: Option<String>,
    pub region_code: Option<String>,
    pub area_code: Option<i32>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub dma_code: Option<i32>,
    pub postal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanMatch {
    pub _shodan: Option<ShodanMeta>,
    pub product: Option<String>,
    pub ip: Option<u32>,
    pub ip_str: Option<String>,
    pub port: Option<u16>,
    pub transport: Option<String>,
    pub hostnames: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub timestamp: Option<String>,
    pub data: Option<String>,
    pub org: Option<String>,
    pub isp: Option<String>,
    pub asn: Option<String>,
    pub os: Option<String>,
    pub location: Option<ShodanLocation>,
    pub opts: Option<serde_json::Value>,
    pub ssl: Option<serde_json::Value>,
    pub http: Option<serde_json::Value>,
    pub ssh: Option<serde_json::Value>,
    pub snmp: Option<serde_json::Value>,
    pub mongodb: Option<serde_json::Value>,
    pub elastic: Option<serde_json::Value>,
    pub vulns: Option<serde_json::Value>,
    pub cpe: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub hash: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShodanSearchResponse {
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub matches: Vec<ShodanMatch>,
    pub error: Option<String>,
}