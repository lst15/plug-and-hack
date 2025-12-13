mod client;
mod queries;

pub use queries::ShodanQuery;

use crate::core::osint_provider::OsintProvider;
use crate::models::query::Query; // ←←← IMPORTANTE: traz o trait para o escopo
use crate::models::shodan::ShodanSearchResponse;
use crate::providers::shodan::client::ShodanClient;
use crate::utils::error::Result;

pub struct ShodanProvider {
    client: ShodanClient,
}

impl ShodanProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: ShodanClient::new(api_key),
        }
    }
}

#[async_trait::async_trait]
impl OsintProvider for ShodanProvider {
    type QueryType = ShodanQuery; // agora não precisa de alias
    type ResultType = ShodanSearchResponse;

    async fn search(&self, query: Self::QueryType) -> Result<Self::ResultType> {
        self.client.search(&query.to_string()).await // ← usa Query::to_string()
    }

    async fn search_many(&self, queries: Vec<Self::QueryType>) -> Result<Vec<Self::ResultType>> {
        let raw_queries: Vec<String> = queries.into_iter().map(|q| q.to_string()).collect();
        self.client.search_many(&raw_queries).await
    }
}