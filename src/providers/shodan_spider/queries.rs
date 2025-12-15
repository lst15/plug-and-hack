// src/providers/shodan_spider/queries.rs
use crate::models::query::Query;

#[derive(Debug, Clone)]
pub struct ShodanSpiderQuery {
    pub raw_query: String,
}

impl Query for ShodanSpiderQuery {
    fn to_string(&self) -> String {
        self.raw_query.clone()
    }
}

impl ShodanSpiderQuery {
    pub fn new(query: &str) -> Self {
        Self {
            raw_query: query.to_string(),
        }
    }

    pub fn from_cve(cve: &str) -> Self {
        // Normaliza para maiúsculas (CVE-2025-55182)
        let cve = cve.trim().to_uppercase();
        Self::new(&format!("vuln:{}", cve))
    }
}