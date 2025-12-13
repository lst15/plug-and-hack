// src/providers/shodan/queries.rs
use crate::models::query::Query;

#[derive(Debug, Default)]
pub struct ShodanQuery {
    pub all: Option<String>,
    pub asn: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    // ... adicione todos os campos que desejar (ou simplifique para Map<String, String>)
    pub http_component: Vec<String>,
    pub http_title: Option<String>,
    pub vuln: Option<String>,
    // ... outros campos relevantes
}

impl Query for ShodanQuery {
    fn to_string(&self) -> String {
        let mut parts = Vec::new();

        macro_rules! push_opt {
            ($field:expr, $val:expr) => {
                if let Some(v) = $val {
                    parts.push(format!("{}:{}", $field, v));
                }
            };
        }

        macro_rules! push_vec {
            ($field:expr, $vals:expr) => {
                for v in $vals {
                    parts.push(format!("{}:{}", $field, v));
                }
            };
        }

        push_opt!("all", &self.all);
        push_opt!("asn", &self.asn);
        push_opt!("city", &self.city);
        push_opt!("country", &self.country);
        push_opt!("vuln", &self.vuln);
        push_opt!("http.title", &self.http_title);
        push_vec!("http.component", &self.http_component);

        parts.join(" ")
    }
}

impl ShodanQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_http_component(mut self, comp: &str) -> Self {
        self.http_component.push(comp.to_string());
        self
    }

    pub fn with_vuln(mut self, cve: &str) -> Self {
        self.vuln = Some(cve.to_string());
        self
    }

    pub fn with_country(mut self, country: &str) -> Self {
        self.country = Some(country.to_string());
        self
    }
}