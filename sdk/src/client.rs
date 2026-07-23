use reqwest::Client;
use uuid::Uuid;

use crate::config::{PagBankConfig, Service};
use crate::error::PagBankError;

#[derive(Clone)]
pub struct PagBankClient {
    http: Client,
    pub config: PagBankConfig,
}

#[derive(Debug, Default)]
pub struct RequestOptions {
    pub idempotency_key: Option<String>,
}

impl PagBankClient {
    pub fn new(config: PagBankConfig) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("falha ao criar HTTP client");
        Self { http, config }
    }

    pub fn base_url(&self, service: Service) -> String {
        service.base_url(&self.config.environment).to_string()
    }

    pub fn main_url(&self) -> String {
        self.base_url(Service::Main)
    }

    pub fn recurring_url(&self) -> String {
        self.base_url(Service::Recurring)
    }

    pub fn secure_url(&self) -> String {
        self.base_url(Service::Secure)
    }

    pub async fn get(
        &self,
        service: Service,
        path: &str,
    ) -> Result<reqwest::Response, PagBankError> {
        let url = format!("{}{}", self.base_url(service), path);
        let token = self.resolve_token(service)?;
        let resp = self
            .http
            .get(&url)
            .header("Authorization", &token)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn post(
        &self,
        service: Service,
        path: &str,
        body: &serde_json::Value,
        opts: &RequestOptions,
    ) -> Result<reqwest::Response, PagBankError> {
        let url = format!("{}{}", self.base_url(service), path);
        let token = self.resolve_token(service)?;
        let idempotency = opts
            .idempotency_key
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let resp = self
            .http
            .post(&url)
            .header("Authorization", &token)
            .header("x-idempotency-key", &idempotency)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn put(
        &self,
        service: Service,
        path: &str,
        body: &serde_json::Value,
        opts: &RequestOptions,
    ) -> Result<reqwest::Response, PagBankError> {
        let url = format!("{}{}", self.base_url(service), path);
        let token = self.resolve_token(service)?;
        let idempotency = opts
            .idempotency_key
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        let resp = self
            .http
            .put(&url)
            .header("Authorization", &token)
            .header("x-idempotency-key", &idempotency)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    pub async fn delete(
        &self,
        service: Service,
        path: &str,
    ) -> Result<reqwest::Response, PagBankError> {
        let url = format!("{}{}", self.base_url(service), path);
        let token = self.resolve_token(service)?;
        let resp = self
            .http
            .delete(&url)
            .header("Authorization", &token)
            .send()
            .await?;
        self.handle_response(resp).await
    }

    async fn handle_response(
        &self,
        resp: reqwest::Response,
    ) -> Result<reqwest::Response, PagBankError> {
        let status = resp.status().as_u16();
        if resp.status().is_success() {
            return Ok(resp);
        }

        let body = resp.text().await.unwrap_or_default();

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
            let code = json
                .get("error_code")
                .or_else(|| json.get("code"))
                .and_then(|v| v.as_str())
                .or_else(|| {
                    json.get("error_messages")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| arr.first())
                        .and_then(|e| e.get("code"))
                        .and_then(|v| v.as_str())
                })
                .unwrap_or("UNKNOWN")
                .to_string();

            let message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .or_else(|| json.get("error_message"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    json.get("error_messages")
                        .and_then(|v| v.as_array())
                        .and_then(|arr| {
                            let parts: Vec<String> = arr
                                .iter()
                                .filter_map(|e| {
                                    let desc =
                                        e.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                    let param = e.get("parameter_name").and_then(|v| v.as_str());
                                    match param {
                                        Some(p) => Some(format!("{p}: {desc}")),
                                        None if !desc.is_empty() => Some(desc.to_string()),
                                        None => None,
                                    }
                                })
                                .collect();
                            if parts.is_empty() {
                                None
                            } else {
                                Some(parts.join("; "))
                            }
                        })
                        .unwrap_or(body.clone())
                });

            return Err(PagBankError::Api {
                status,
                code,
                message,
            });
        }

        Err(PagBankError::ApiRaw { status, body })
    }

    fn resolve_token(&self, service: Service) -> Result<String, PagBankError> {
        match service {
            Service::Recurring => {
                let token = self
                    .config
                    .recurring_token
                    .as_deref()
                    .unwrap_or(&self.config.token);
                if token.is_empty() {
                    return Err(PagBankError::NoRecurringToken);
                }
                Ok(format!("Bearer {token}"))
            }
            _ => {
                if self.config.token.is_empty() {
                    return Err(PagBankError::NoToken);
                }
                Ok(format!("Bearer {}", self.config.token))
            }
        }
    }
}
