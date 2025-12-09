use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ZoomEyeLoginResponse {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ZoomEyeQuotaInfo {
    pub remain_free: Option<u64>,
    pub remain_pay: Option<u64>,
    pub remain_total: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ZoomEyeResourceInfo {
    pub plan: Option<String>,
    pub resources: Option<serde_json::Value>,
    pub quota: Option<ZoomEyeQuotaInfo>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ZoomEyeFilters {
    pub keywords: Vec<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub app: Option<String>,
    pub service: Option<String>,
    pub port: Option<String>,
    pub os: Option<String>,
    pub cidr: Option<String>,
}

impl ZoomEyeFilters {
    pub fn to_query(&self) -> String {
        let mut parts = Vec::new();

        if !self.keywords.is_empty() {
            parts.push(self.keywords.join(" "));
        }

        macro_rules! push_filter {
            ($field:ident, $name:expr) => {
                if let Some(value) = &self.$field {
                    if !value.trim().is_empty() {
                        parts.push(format!("{}:{}", $name, value));
                    }
                }
            };
        }

        push_filter!(country, "country");
        push_filter!(city, "city");
        push_filter!(app, "app");
        push_filter!(service, "service");
        push_filter!(port, "port");
        push_filter!(os, "os");
        push_filter!(cidr, "cidr");

        parts
            .into_iter()
            .filter(|part| !part.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoomEyeGeoInfo {
    pub continent: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    pub province: Option<serde_json::Value>,
    pub city: Option<serde_json::Value>,
    pub organization: Option<serde_json::Value>,
    pub asn: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoomEyePortInfo {
    pub hostname: Option<Vec<String>>,
    pub service: Option<String>,
    pub app: Option<String>,
    pub port: Option<u16>,
    pub banner: Option<String>,
    pub transport: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoomEyeMatch {
    pub ip: Option<String>,
    pub portinfo: Option<ZoomEyePortInfo>,
    pub geoinfo: Option<ZoomEyeGeoInfo>,
    pub timestamp: Option<String>,
    pub protocols: Option<Vec<String>>,
    pub os: Option<String>,
    pub device: Option<String>,
    pub tags: Option<Vec<String>>,
    pub raw_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoomEyeSearchResponse {
    pub total: Option<u64>,
    pub matches: Option<Vec<ZoomEyeMatch>>,
}

pub struct ZoomEye {
    token: String,
}

impl ZoomEye {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub async fn login(username: String, password: String) -> ZoomEyeLoginResponse {
        let body = serde_json::json!({
            "username": username,
            "password": password,
        });

        let rsp = reqwest::Client::new()
            .post("https://api.zoomeye.org/user/login")
            .json(&body)
            .send()
            .await
            .expect("http error");

        rsp.json().await.expect("json error")
    }

    pub async fn resources_info(&self) -> ZoomEyeResourceInfo {
        let rsp = reqwest::Client::new()
            .get("https://api.zoomeye.org/resources-info")
            .header("Authorization", format!("JWT {}", self.token))
            .send()
            .await
            .expect("http error");

        rsp.json().await.expect("json error")
    }

    pub async fn host_search(
        &self,
        filters: ZoomEyeFilters,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> ZoomEyeSearchResponse {
        let mut url = format!(
            "https://api.zoomeye.org/host/search?query={}",
            urlencoding::encode(&filters.to_query())
        );

        if let Some(page) = page {
            url.push_str(&format!("&page={page}"));
        }

        if let Some(page_size) = page_size {
            url.push_str(&format!("&pageSize={page_size}"));
        }

        let rsp = reqwest::Client::new()
            .get(url)
            .header("Authorization", format!("JWT {}", self.token))
            .send()
            .await
            .expect("http error");

        rsp.json().await.expect("json error")
    }
}
