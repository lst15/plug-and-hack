use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AccountProfile {
    member: bool,
    display_name: Option<String>,
    credits: u64,
    username: Option<String>,
    created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanMeta {
    pub module: Option<String>,
    pub crawler: Option<String>,
    pub id: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanLocation {
    pub city: Option<String>,
    pub region_code: Option<String>,
    pub area_code: Option<i32>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub dma_code: Option<i32>,
    pub postal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanMatch {
    pub _shodan: Option<ShodanMeta>,
    pub product: Option<String>,
    pub ip: Option<u32>,
    pub ip_str: Option<String>,
    pub port: Option<u16>,
    pub transport: Option<String>,
    pub hostnames: Option<Vec<String>>,
    pub domains: Option<Vec<String>>,
    pub timestamp: Option<String>,
    pub data: Option<String>,
    pub org: Option<String>,
    pub isp: Option<String>,
    pub asn: Option<String>,
    pub os: Option<String>,

    pub location: Option<ShodanLocation>,
    pub opts: Option<serde_json::Value>,
    pub ssl: Option<serde_json::Value>,
    pub http: Option<serde_json::Value>,
    pub ssh: Option<serde_json::Value>,
    pub snmp: Option<serde_json::Value>,
    pub mongodb: Option<serde_json::Value>,
    pub elastic: Option<serde_json::Value>,
    pub vulns: Option<serde_json::Value>,

    pub cpe: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub hash: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShodanSearchResponse {
    pub total: u64,
    pub matches: Vec<ShodanMatch>,
}

pub struct Shodan {
    key: String,
}

//https://www.shodan.io/search/filters
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Filters {
    pub all: Option<String>,
    pub asn: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub cpe: Option<String>,
    pub device: Option<String>,
    pub geo: Option<String>,
    pub has_ipv6: Option<String>,
    pub has_screenshot: Option<String>,
    pub has_ssl: Option<String>,
    pub has_vuln: Option<String>,
    pub hash: Option<String>,
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub isp: Option<String>,
    pub net: Option<String>,
    pub org: Option<String>,
    pub os: Option<String>,
    pub port: Option<String>,
    pub postal: Option<String>,
    pub product: Option<String>,
    pub region: Option<String>,
    pub scan: Option<String>,
    pub shodan_module: Option<String>,
    pub state: Option<String>,
    pub version: Option<String>,

    // screenshots
    pub screenshot_hash: Option<String>,
    pub screenshot_label: Option<String>,

    // cloud
    pub cloud_provider: Option<String>,
    pub cloud_region: Option<String>,
    pub cloud_service: Option<String>,

    // http
    pub http_component: Vec<String>,
    pub http_component_category: Option<String>,
    pub http_dom_hash: Option<String>,
    pub http_favicon_hash: Option<String>,
    pub http_headers_hash: Option<String>,
    pub http_html: Vec<String>,
    pub http_html_hash: Option<String>,
    pub http_robots_hash: Option<String>,
    pub http_securitytxt: Option<String>,
    pub http_server_hash: Option<String>,
    pub http_status: Option<String>,
    pub http_title: Option<String>,
    pub http_title_hash: Option<String>,
    pub http_waf: Option<String>,

    // bitcoin
    pub bitcoin_ip: Option<String>,
    pub bitcoin_ip_count: Option<String>,
    pub bitcoin_port: Option<String>,
    pub bitcoin_version: Option<String>,

    // restricted
    pub tag: Option<String>,
    pub vuln: Option<String>,

    // snmp
    pub snmp_contact: Option<String>,
    pub snmp_location: Option<String>,
    pub snmp_name: Option<String>,

    // ssl
    pub ssl: Option<String>,
    pub ssl_alpn: Option<String>,
    pub ssl_cert_alg: Option<String>,
    pub ssl_cert_expired: Option<String>,
    pub ssl_cert_extension: Option<String>,
    pub ssl_cert_fingerprint: Option<String>,
    pub ssl_cert_issuer_cn: Option<String>,
    pub ssl_cert_pubkey_bits: Option<String>,
    pub ssl_cert_pubkey_type: Option<String>,
    pub ssl_cert_serial: Option<String>,
    pub ssl_cert_subject_cn: Option<String>,
    pub ssl_chain_count: Option<String>,
    pub ssl_cipher_bits: Option<String>,
    pub ssl_cipher_name: Option<String>,
    pub ssl_cipher_version: Option<String>,
    pub ssl_ja3s: Option<String>,
    pub ssl_jarm: Option<String>,
    pub ssl_version: Option<String>,

    // ntp
    pub ntp_ip: Option<String>,
    pub ntp_ip_count: Option<String>,
    pub ntp_more: Option<String>,
    pub ntp_port: Option<String>,

    // telnet
    pub telnet_do: Option<String>,
    pub telnet_dont: Option<String>,
    pub telnet_option: Option<String>,
    pub telnet_will: Option<String>,
    pub telnet_wont: Option<String>,

    // ssh
    pub ssh_hash: Option<String>,
    pub ssh_type: Option<String>,
}

impl Filters {
    pub fn to_query(&self) -> String {
        let mut q = Vec::new();

        macro_rules! pushf {
            ($field:ident, $real:expr) => {
                for v in &self.$field {
                    q.push(format!("{}:{}", $real, v));
                }
            };
        }

        // básicos
        pushf!(all, "all");
        pushf!(asn, "asn");
        pushf!(city, "city");
        pushf!(country, "country");
        pushf!(cpe, "cpe");
        pushf!(device, "device");
        pushf!(geo, "geo");
        pushf!(has_ipv6, "has_ipv6");
        pushf!(has_screenshot, "has_screenshot");
        pushf!(has_ssl, "has_ssl");
        pushf!(has_vuln, "has_vuln");
        pushf!(hash, "hash");
        pushf!(hostname, "hostname");
        pushf!(ip, "ip");
        pushf!(isp, "isp");
        pushf!(net, "net");
        pushf!(org, "org");
        pushf!(os, "os");
        pushf!(port, "port");
        pushf!(postal, "postal");
        pushf!(product, "product");
        pushf!(region, "region");
        pushf!(scan, "scan");
        pushf!(shodan_module, "shodan.module");
        pushf!(state, "state");
        pushf!(version, "version");

        // screenshots
        pushf!(screenshot_hash, "screenshot.hash");
        pushf!(screenshot_label, "screenshot.label");

        // cloud
        pushf!(cloud_provider, "cloud.provider");
        pushf!(cloud_region, "cloud.region");
        pushf!(cloud_service, "cloud.service");

        // http
        pushf!(http_component, "http.component");
        pushf!(http_component_category, "http.component_category");
        pushf!(http_dom_hash, "http.dom_hash");
        pushf!(http_favicon_hash, "http.favicon.hash");
        pushf!(http_headers_hash, "http.headers_hash");
        pushf!(http_html, "http.html");
        pushf!(http_html_hash, "http.html_hash");
        pushf!(http_robots_hash, "http.robots_hash");
        pushf!(http_securitytxt, "http.securitytxt");
        pushf!(http_server_hash, "http.server.hash");
        pushf!(http_status, "http.status");
        pushf!(http_title, "http.title");
        pushf!(http_title_hash, "http.title_hash");
        pushf!(http_waf, "http.waf");

        // bitcoin
        pushf!(bitcoin_ip, "bitcoin.ip");
        pushf!(bitcoin_ip_count, "bitcoin.ip_count");
        pushf!(bitcoin_port, "bitcoin.port");
        pushf!(bitcoin_version, "bitcoin.version");

        // restricted
        pushf!(tag, "tag");
        pushf!(vuln, "vuln");

        // snmp
        pushf!(snmp_contact, "snmp.contact");
        pushf!(snmp_location, "snmp.location");
        pushf!(snmp_name, "snmp.name");

        // ssl
        pushf!(ssl, "ssl");
        pushf!(ssl_alpn, "ssl.alpn");
        pushf!(ssl_cert_alg, "ssl.cert.alg");
        pushf!(ssl_cert_expired, "ssl.cert.expired");
        pushf!(ssl_cert_extension, "ssl.cert.extension");
        pushf!(ssl_cert_fingerprint, "ssl.cert.fingerprint");
        pushf!(ssl_cert_issuer_cn, "ssl.cert.issuer.cn");
        pushf!(ssl_cert_pubkey_bits, "ssl.cert.pubkey.bits");
        pushf!(ssl_cert_pubkey_type, "ssl.cert.pubkey.type");
        pushf!(ssl_cert_serial, "ssl.cert.serial");
        pushf!(ssl_cert_subject_cn, "ssl.cert.subject.cn");
        pushf!(ssl_chain_count, "ssl.chain.count");
        pushf!(ssl_cipher_bits, "ssl.cipher.bits");
        pushf!(ssl_cipher_name, "ssl.cipher.name");
        pushf!(ssl_cipher_version, "ssl.cipher.version");
        pushf!(ssl_ja3s, "ssl.ja3s");
        pushf!(ssl_jarm, "ssl.jarm");
        pushf!(ssl_version, "ssl.version");

        // ntp
        pushf!(ntp_ip, "ntp.ip");
        pushf!(ntp_ip_count, "ntp.ip_count");
        pushf!(ntp_more, "ntp.more");
        pushf!(ntp_port, "ntp.port");

        // telnet
        pushf!(telnet_do, "telnet.do");
        pushf!(telnet_dont, "telnet.dont");
        pushf!(telnet_option, "telnet.option");
        pushf!(telnet_will, "telnet.will");
        pushf!(telnet_wont, "telnet.wont");

        // ssh
        pushf!(ssh_hash, "ssh.hash");
        pushf!(ssh_type, "ssh.type");

        q.join(" ")
    }
}

impl Shodan {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    pub async fn get_account(self) -> AccountProfile {
        let key = self.key;
        let url = format!("https://api.shodan.io/account/profile?key={key}");

        let rsp = reqwest::get(url).await.expect("ERR");
        let body: AccountProfile = rsp.json().await.expect("ERR");

        body
    }

    pub async fn get_hosts(self, matches: Vec<ShodanMatch>) -> Vec<String> {
        matches.into_iter().map(|x| {
            let module = match x._shodan.as_ref().and_then(|s| s.module.as_ref()).map(|m| m.as_str()) {
                Some("https") => "https",
                _ => "http",
            };

            format!("{}://{}:{}", module, x.ip_str.unwrap(), x.port.unwrap())
        }).collect()
    }

    pub async fn get_hosts_from_responses(
        &self,
        responses: Vec<ShodanSearchResponse>
    ) -> Vec<String> {
        let mut out = Vec::new();

        for rsp in responses {
            for x in rsp.matches {
                let module = match x._shodan
                    .as_ref()
                    .and_then(|s| s.module.as_ref())
                    .map(|m| m.as_str())
                {
                    Some("https") => "https",
                    _ => "http",
                };

                let host = format!("{}://{}:{}", module, x.ip_str.unwrap(), x.port.unwrap());
                out.push(host);
            }
        }

        out
    }




    async fn query_iter<I>(&self, iter: I) -> Vec<ShodanSearchResponse>
    where
        I: IntoIterator<Item = String>,
    {
        let key = &self.key;

        let mut out = Vec::new();

        for q in iter {
            let url = format!(
                "https://api.shodan.io/shodan/host/search?key={key}&query={}",
                urlencoding::encode(&q)
            );

            let rsp = reqwest::get(url).await.expect("http");
            let json: ShodanSearchResponse = rsp.json().await.expect("json");

            out.push(json);
        }

        out
    }

    pub async fn query_raw_many(&self, queries: Vec<String>) -> Vec<ShodanSearchResponse> {
        self.query_iter(queries).await
    }

    pub async fn query_many(&self, filters_list: Vec<Filters>) -> Vec<ShodanSearchResponse> {
        let queries = filters_list.into_iter().map(|f| f.to_query());
        self.query_iter(queries).await
    }

    pub async fn query(&self, filters: Filters) -> ShodanSearchResponse {
        let query = filters.to_query();
        let mut out = self.query_iter(std::iter::once(query)).await;

        out.remove(0)
    }
}
