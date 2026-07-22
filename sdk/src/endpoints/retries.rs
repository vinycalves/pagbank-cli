use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::invoice::*;

pub async fn get(client: &PagBankClient, id: &str) -> Result<Retry, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/retries/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Retry, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            &format!("/retries/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn retry_now(client: &PagBankClient, id: &str) -> Result<Retry, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/retries/{id}/retry"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
