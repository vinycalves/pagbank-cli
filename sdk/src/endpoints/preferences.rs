use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;

pub async fn get_preferences(client: &PagBankClient) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .get(Service::Recurring, "/notification-preferences")
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn update_preferences(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            "/notification-preferences",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_encryption_key(client: &PagBankClient) -> Result<serde_json::Value, PagBankError> {
    let resp = client.get(Service::Recurring, "/encryption-key").await?;
    crate::models::parse_response(resp).await
}

pub async fn create_encryption_key(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            "/encryption-key",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
