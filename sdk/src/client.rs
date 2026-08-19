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

    /// Faz um GET em uma URL absoluta (ex: link QRCODE.BASE64 retornado pela API)
    /// e devolve o corpo como texto. Reutiliza o client e o token já resolvidos.
    pub async fn get_url_text(&self, url: &str) -> Result<String, PagBankError> {
        let token = self.resolve_token(Service::Main)?;
        let resp = self
            .http
            .get(url)
            .header("Authorization", &token)
            .send()
            .await?;
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        if (200..300).contains(&status) {
            Ok(body)
        } else {
            Err(PagBankError::ApiRaw { status, body })
        }
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
        Err(parse_error_body(status, &body))
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

fn parse_error_body(status: u16, body: &str) -> PagBankError {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
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
                    .unwrap_or_else(|| body.to_string())
            });

        return PagBankError::Api {
            status,
            code,
            message,
        };
    }

    PagBankError::ApiRaw {
        status,
        body: body.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Environment, PagBankConfig};

    fn client_with(token: &str, recurring: Option<&str>) -> PagBankClient {
        PagBankClient {
            http: reqwest::Client::new(),
            config: PagBankConfig {
                environment: Environment::Sandbox,
                token: token.to_string(),
                recurring_token: recurring.map(String::from),
                client_id: None,
                client_secret: None,
            },
        }
    }

    #[test]
    fn resolve_token_main_ok() {
        let c = client_with("tok123", None);
        assert_eq!(c.resolve_token(Service::Main).unwrap(), "Bearer tok123");
    }

    #[test]
    fn resolve_token_main_missing() {
        let c = client_with("", None);
        assert!(matches!(
            c.resolve_token(Service::Main),
            Err(PagBankError::NoToken)
        ));
    }

    #[test]
    fn resolve_token_recurring_uses_own() {
        let c = client_with("main", Some("rec"));
        assert_eq!(c.resolve_token(Service::Recurring).unwrap(), "Bearer rec");
    }

    #[test]
    fn resolve_token_recurring_falls_back_to_main() {
        let c = client_with("main", None);
        assert_eq!(c.resolve_token(Service::Recurring).unwrap(), "Bearer main");
    }

    #[test]
    fn resolve_token_recurring_missing() {
        let c = client_with("", None);
        assert!(matches!(
            c.resolve_token(Service::Recurring),
            Err(PagBankError::NoRecurringToken)
        ));
    }

    #[test]
    fn parse_error_body_json() {
        let body = r#"{"error_code":"40002","message":"campo inválido"}"#;
        match parse_error_body(400, body) {
            PagBankError::Api {
                status,
                code,
                message,
            } => {
                assert_eq!(status, 400);
                assert_eq!(code, "40002");
                assert_eq!(message, "campo inválido");
            }
            other => panic!("esperado Api, obtido {other}"),
        }
    }

    #[test]
    fn parse_error_body_error_messages() {
        let body = r#"{
            "error_messages": [
                {"code": "40001", "description": "obrigatório", "parameter_name": "name"}
            ]
        }"#;
        match parse_error_body(422, body) {
            PagBankError::Api {
                status,
                code,
                message,
            } => {
                assert_eq!(status, 422);
                assert_eq!(code, "40001");
                assert!(message.contains("name"));
                assert!(message.contains("obrigatório"));
            }
            other => panic!("esperado Api, obtido {other}"),
        }
    }

    #[test]
    fn parse_error_body_raw() {
        let body = "texto plano";
        match parse_error_body(500, body) {
            PagBankError::ApiRaw { status, body: b } => {
                assert_eq!(status, 500);
                assert_eq!(b, "texto plano");
            }
            other => panic!("esperado ApiRaw, obtido {other}"),
        }
    }
}
