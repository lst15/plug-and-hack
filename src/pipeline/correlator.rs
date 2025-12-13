// src/pipeline/correlator.rs
use crate::models::shodan::ShodanMatch;
use crate::models::target::Target;
use std::collections::HashSet;

pub struct Correlator;

impl Correlator {
    /// Converte resultados brutos (ex: ShodanMatch) em Targets unificados
    pub fn correlate_shodan(matches: &[ShodanMatch]) -> Vec<Target> {
        let mut seen = HashSet::new();
        let mut targets = Vec::new();

        for m in matches {
            if let (Some(ip), Some(port)) = (&m.ip_str, m.port) {
                let key = format!("{}:{}", ip, port);
                if seen.contains(&key) {
                    continue;
                }
                seen.insert(key);

                targets.push(Target {
                    ip: ip.clone(),
                    port,
                    service: m.product.clone().or(m._shodan.as_ref().and_then(|s| s.module.clone())),
                    os: m.os.clone(),
                    domains: m.domains.clone().unwrap_or_default(),
                    vulns: extract_vulns(&m.vulns),
                    raw_data: serde_json::to_value(m).ok(),
                });
            }
        }

        targets
    }
}

fn extract_vulns(vulns: &Option<serde_json::Value>) -> Vec<String> {
    if let Some(map) = vulns.as_ref().and_then(|v| v.as_object()) {
        map.keys().cloned().collect()
    } else {
        Vec::new()
    }
}