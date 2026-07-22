use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::checkout::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Checkout, PagBankError> {
    let resp = client
        .post(Service::Main, "/checkouts", body, &RequestOptions::default())
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Checkout, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/checkouts/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn activate(
    client: &PagBankClient,
    id: &str,
) -> Result<Checkout, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .post(
            Service::Main,
            &format!("/checkouts/{id}/activate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn deactivate(
    client: &PagBankClient,
    id: &str,
) -> Result<Checkout, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .post(
            Service::Main,
            &format!("/checkouts/{id}/deactivate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
