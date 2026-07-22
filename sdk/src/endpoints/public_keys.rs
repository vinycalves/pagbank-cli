use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::PublicKey;

pub async fn create(client: &PagBankClient) -> Result<PublicKey, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .post(
            Service::Main,
            "/public-keys",
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<PublicKey, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/public-keys/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<PublicKey, PagBankError> {
    let resp = client
        .put(
            Service::Main,
            &format!("/public-keys/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
