// src/pipeline/collector.rs
use crate::core::osint_provider::OsintProvider;
use crate::models::query::Query;
use crate::utils::error::Result;

pub struct Collector;

impl Collector {
    pub async fn collect_from_provider<P, Q>(
        provider: &P,
        queries: Vec<Q>,
    ) -> Result<Vec<P::ResultType>>
    where
        P: OsintProvider<QueryType = Q>,
        Q: Query,
    {
        provider.search_many(queries).await
    }
}