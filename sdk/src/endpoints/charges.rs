use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::charge::*;

pub async fn get(client: &PagBankClient, id: &str) -> Result<Charge, PagBankError> {
    let resp = client.get(Service::Main, &format!("/charges/{id}")).await?;
    crate::models::parse_response(resp).await
}

pub async fn capture(
    client: &PagBankClient,
    charge_id: &str,
    body: Option<&serde_json::Value>,
) -> Result<Charge, PagBankError> {
    let empty_body = serde_json::json!({});
    let payload = body.unwrap_or(&empty_body);
    let resp = client
        .post(
            Service::Main,
            &format!("/charges/{charge_id}/capture"),
            payload,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn cancel(client: &PagBankClient, charge_id: &str) -> Result<Charge, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .post(
            Service::Main,
            &format!("/charges/{charge_id}/cancel"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn create_3ds_session(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/authentication-sessions",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_costs(
    client: &PagBankClient,
    charge_id: &str,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/charges/{charge_id}/costs"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn store_card(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .post(Service::Main, "/cards", body, &RequestOptions::default())
        .await?;
    crate::models::parse_response(resp).await
}
