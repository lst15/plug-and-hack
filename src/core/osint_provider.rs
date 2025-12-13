// src/core/osint_provider.rs
use crate::models::query::Query;
// src/core/osint_provider.rs
use crate::utils::error::Result; // <-- use seu próprio Result
use async_trait::async_trait;

#[async_trait]
pub trait OsintProvider {
    type QueryType: crate::models::query::Query;
    type ResultType;

    async fn search(&self, query: Self::QueryType) -> Result<Self::ResultType>;
    async fn search_many(&self, queries: Vec<Self::QueryType>) -> Result<Vec<Self::ResultType>>;
}