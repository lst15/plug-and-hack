use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum MetasploitError {
    Io(std::io::Error),
    Encode(rmp_serde::encode::Error),
    Decode(rmp_serde::decode::Error),
    InvalidResponse(String),
}

impl From<std::io::Error> for MetasploitError {
    fn from(err: std::io::Error) -> Self {
        MetasploitError::Io(err)
    }
}

impl From<rmp_serde::encode::Error> for MetasploitError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        MetasploitError::Encode(err)
    }
}

impl From<rmp_serde::decode::Error> for MetasploitError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        MetasploitError::Decode(err)
    }
}

#[derive(Serialize)]
struct RpcRequest<'a> {
    method: &'a str,
    params: &'a [Value],
    id: u32,
}

#[derive(Deserialize)]
struct AuthResponse {
    result: String,
    token: Option<String>,
    error: Option<Value>,
}

pub struct MetasploitClient {
    stream: TcpStream,
    token: String,
    request_id: u32,
}

impl MetasploitClient {
    pub async fn new(
        host: String,
        port: u16,
        username: String,
        password: String,
    ) -> Result<Self, MetasploitError> {
        let address = format!("{host}:{port}");
        let stream = TcpStream::connect(&address).await.map_err(|err| {
            let msg = format!("Failed to connect to msfgrpc service at {address}: {err}");
            MetasploitError::Io(std::io::Error::new(err.kind(), msg))
        })?;

        let mut client = Self {
            stream,
            token: String::new(),
            request_id: 1,
        };

        let token = client.authenticate(username, password).await?;

        client.token = token;
        Ok(client)
    }

    async fn authenticate(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, MetasploitError> {
        let params = vec![Value::String(username), Value::String(password)];
        let response = self.send_rpc("auth.login", params).await?;

        let parsed: AuthResponse = serde_json::from_value(response)
            .map_err(|err| MetasploitError::InvalidResponse(err.to_string()))?;

        match parsed.result.as_str() {
            "success" => parsed
                .token
                .ok_or_else(|| MetasploitError::InvalidResponse("Missing token field".into())),
            _ => Err(MetasploitError::InvalidResponse(
                parsed
                    .error
                    .map(|err| err.to_string())
                    .unwrap_or_else(|| "Authentication was not successful".into()),
            )),
        }
    }

    pub async fn call(&mut self, method: &str, mut params: Vec<Value>) -> Value {
        params.insert(0, Value::String(self.token.clone()));
        self.send_rpc(method, params)
            .await
            .unwrap_or_else(|err| panic!("RPC call failed: {err:?}"))
    }

    async fn send_rpc(
        &mut self,
        method: &str,
        params: Vec<Value>,
    ) -> Result<Value, MetasploitError> {
        let request = RpcRequest {
            method,
            params: &params,
            id: self.request_id,
        };

        self.request_id = self.request_id.wrapping_add(1);

        let payload = rmp_serde::to_vec(&request)?;
        let length: u32 = payload
            .len()
            .try_into()
            .map_err(|_| MetasploitError::InvalidResponse("Payload too large".into()))?;

        let mut frame = Vec::with_capacity(4 + payload.len());
        frame.extend_from_slice(&length.to_be_bytes());
        frame.extend_from_slice(&payload);

        self.stream.write_all(&frame).await?;
        self.stream.flush().await?;

        let mut length_buf = [0u8; 4];
        self.stream.read_exact(&mut length_buf).await?;
        let expected_length = u32::from_be_bytes(length_buf) as usize;

        let mut response_buf = vec![0u8; expected_length];
        self.stream.read_exact(&mut response_buf).await?;

        let response: Value = rmp_serde::from_slice(&response_buf)?;
        Ok(response)
    }
}
