// src/providers/shodan/client.rs
use crate::models::shodan::{ShodanSearchResponse, AccountProfile};
use crate::utils::error::{Error, Result};
use urlencoding::encode;
use reqwest;

pub struct ShodanClient {
    api_key: String,
    client: reqwest::Client,
}

impl ShodanClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn account_profile(&self) -> Result<AccountProfile> {
        let url = format!(
            "https://api.shodan.io/account/profile?key={}",
            self.api_key
        );
        let resp = self.client.get(&url).send().await.expect("ERR");
        let profile = resp.json().await.expect("ERR");
        Ok(profile)
    }

    pub async fn search(&self, query: &str) -> Result<ShodanSearchResponse> {
        let url = format!(
            "https://api.shodan.io/shodan/host/search?key={}&query={}",
            self.api_key,
            encode(query)
        );
        let resp = self.client.get(&url).send().await.expect("ERR");
        println!("{:?}", resp);
        let json: ShodanSearchResponse = resp.json().await.expect("ERR");

        if let Some(err) = &json.error {
            return Err(Error::ShodanApi(err.clone()));
        }

        Ok(json)
    }

    pub async fn search_many(&self, queries: &[String]) -> Result<Vec<ShodanSearchResponse>> {
        let mut results = Vec::new();
        for q in queries {
            results.push(self.search(q).await?);
        }
        Ok(results)
    }
}