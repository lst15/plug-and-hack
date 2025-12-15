// src/providers/shodan_spider/mod.rs

mod client;
mod queries;

pub use queries::ShodanSpiderQuery;

use crate::core::osint_provider::OsintProvider;
use crate::models::query::Query;
use crate::models::target::Target;
use crate::providers::shodan_spider::client::ShodanSpiderClient;
use crate::utils::error::Result;

pub struct ShodanSpiderProvider {
    client: ShodanSpiderClient,
}

impl ShodanSpiderProvider {
    pub fn new() -> Self {
        Self {
            client: ShodanSpiderClient::new(),
        }
    }
}

#[async_trait::async_trait]
impl OsintProvider for ShodanSpiderProvider {
    type QueryType = ShodanSpiderQuery;
    type ResultType = Vec<Target>;

    async fn search(&self, query: Self::QueryType) -> Result<Self::ResultType> {
        let ips = self.client.search_ips(&query.to_string()).await?;
        let targets = ips
            .into_iter()
            .map(|ip| Target {
                ip,
                port: 80,
                service: Some("http".to_string()),
                os: None,
                domains: vec![],
                vulns: vec![],
                raw_data: None,
            })
            .collect();
        Ok(targets)
    }

    async fn search_many(&self, queries: Vec<Self::QueryType>) -> Result<Vec<Self::ResultType>> {
        let mut all = Vec::new();
        for q in queries {
            all.push(self.search(q).await?);
        }
        Ok(all)
    }
}