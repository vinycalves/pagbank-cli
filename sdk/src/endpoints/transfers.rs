use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::transfer::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
    opts: &RequestOptions,
) -> Result<Transfer, PagBankError> {
    let resp = client
        .post(Service::Transfer, "/transfers", body, opts)
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Transfer, PagBankError> {
    let resp = client
        .get(Service::Transfer, &format!("/transfers/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Transfer>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/transfers".to_string()
    } else {
        format!("/transfers?{query}")
    };
    let resp = client.get(Service::Transfer, &path).await?;
    crate::models::parse_list(resp).await
}
