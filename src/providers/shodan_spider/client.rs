// src/providers/shodan_spider/client.rs

use crate::utils::error::{Result, Error};
use regex::Regex;
use reqwest::header::{HeaderMap, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE};
use urlencoding::encode;

pub struct ShodanSpiderClient {
    client: reqwest::Client,
    user_agents: Vec<String>,
}

impl ShodanSpiderClient {
    pub fn new() -> Self {
        let user_agents = vec![
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Firefox/122.0".to_string(),
        ];

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "text/html,application/xhtml+xml".parse().unwrap());
        headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .gzip(true)
            .build()
            .unwrap();

        Self { client, user_agents }
    }

    pub async fn search_ips(&self, query: &str) -> Result<Vec<String>> {
        let encoded = encode(query);
        let url = format!("https://www.shodan.io/search/facet?query={}&facet=ip", encoded);

        // Seleciona User-Agent aleatório
        use rand::Rng;
        let ua = &self.user_agents[rand::thread_rng().gen_range(0..self.user_agents.len())];

        let resp = self
            .client
            .get(&url)
            .header(USER_AGENT, ua)
            .send()
            .await
            .map_err(|e| Error::Other(format!("HTTP error: {}", e)))?;

        let body = resp.text().await.map_err(|e| Error::Other(format!("Body error: {}", e)))?;
        println!("http://{}", body);
        let ips = extract_public_ips(&body);
        Ok(ips)
    }
}

fn extract_public_ips(html: &str) -> Vec<String> {
    let ip_re = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap();
    let mut ips = Vec::new();

    for ip_match in ip_re.find_iter(html) {
        let s = ip_match.as_str();
        if is_public_ip(s) {
            ips.push(s.to_string());
        }
    }

    // Remove duplicatas mantendo ordem
    ips.sort_unstable();
    ips.dedup();
    ips
}

fn is_public_ip(ip: &str) -> bool {
    if let Ok(addr) = ip.parse::<std::net::Ipv4Addr>() {
        // Verifica se é privado (RFC 1918)
        let octets = addr.octets();
        match octets {
            [10, _, _, _] => false, // 10.0.0.0/8
            [172, b, _, _] if b >= 16 && b <= 31 => false, // 172.16.0.0/12
            [192, 168, _, _] => false, // 192.168.0.0/16
            [127, _, _, _] => false, // loopback
            [169, 254, _, _] => false, // link-local
            _ => true, // público
        }
    } else {
        false // não é IPv4 válido
    }
}