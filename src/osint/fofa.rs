use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct FofaUserInfo {
    pub email: Option<String>,
    pub username: Option<String>,
    pub fcoin: Option<u64>,
    pub fofacli: Option<bool>,
    pub isvip: Option<bool>,
    pub vip_level: Option<u32>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FofaFilters {
    pub keyword: Option<String>,
    pub host: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub protocol: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub asn: Option<String>,
    pub title: Option<String>,
    pub header: Option<String>,
    pub body: Option<String>,
    pub banner: Option<String>,
    pub os: Option<String>,
    pub server: Option<String>,
    pub app: Option<String>,
    pub icp: Option<String>,
    pub fid: Option<String>,
    pub cname: Option<String>,
    pub cpe: Option<String>,
    pub status_code: Option<String>,
    pub cert: Option<String>,
    pub org: Option<String>,
    pub icon_hash: Option<String>,
    pub base_query: Option<String>,
}

impl FofaFilters {
    pub fn to_query(&self) -> String {
        let mut parts = Vec::new();

        macro_rules! push_filter {
            ($field:ident, $name:expr) => {
                if let Some(value) = &self.$field {
                    parts.push(format!("{}:\"{}\"", $name, value));
                }
            };
        }

        if let Some(keyword) = &self.keyword {
            parts.push(keyword.clone());
        }

        push_filter!(host, "host");
        push_filter!(ip, "ip");
        push_filter!(port, "port");
        push_filter!(protocol, "protocol");
        push_filter!(country, "country");
        push_filter!(region, "region");
        push_filter!(city, "city");
        push_filter!(asn, "asn");
        push_filter!(title, "title");
        push_filter!(header, "header");
        push_filter!(body, "body");
        push_filter!(banner, "banner");
        push_filter!(os, "os");
        push_filter!(server, "server");
        push_filter!(app, "app");
        push_filter!(icp, "icp");
        push_filter!(fid, "fid");
        push_filter!(cname, "cname");
        push_filter!(cpe, "cpe");
        push_filter!(status_code, "status_code");
        push_filter!(cert, "cert");
        push_filter!(org, "org");
        push_filter!(icon_hash, "icon_hash");

        if let Some(query) = &self.base_query {
            parts.push(query.clone());
        }

        parts
            .into_iter()
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FofaSearchResponse {
    pub error: Option<bool>,
    pub errmsg: Option<String>,
    pub query: Option<String>,
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub total: Option<u64>,
    pub mode: Option<String>,
    pub results: Option<Vec<Vec<serde_json::Value>>>,
}

pub struct Fofa {
    email: String,
    key: String,
}

impl Fofa {
    pub fn new(email: String, key: String) -> Self {
        Self { email, key }
    }

    pub async fn get_account(&self) -> FofaUserInfo {
        let url = format!(
            "https://fofa.info/api/v1/info/my?email={}&key={}",
            self.email, self.key
        );
        let rsp = reqwest::get(url).await.expect("http error");
        rsp.json().await.expect("json error")
    }

    pub async fn query(
        &self,
        filters: FofaFilters,
        page: Option<u32>,
        size: Option<u32>,
        fields: Option<Vec<String>>,
        full: Option<bool>,
    ) -> FofaSearchResponse {
        let query = filters.to_query();
        let encoded_query = base64::engine::general_purpose::STANDARD.encode(query);

        let mut url = format!(
            "https://fofa.info/api/v1/search/all?email={}&key={}&qbase64={}",
            self.email, self.key, encoded_query
        );

        if let Some(page) = page {
            url.push_str(&format!("&page={}", page));
        }

        if let Some(size) = size {
            url.push_str(&format!("&size={}", size));
        }

        if let Some(fields) = fields {
            if !fields.is_empty() {
                url.push_str(&format!("&fields={}", fields.join(",")));
            }
        }

        if let Some(full) = full {
            url.push_str(&format!("&full={}", full));
        }

        let rsp = reqwest::get(url).await.expect("http error");
        rsp.json().await.expect("json error")
    }
}
