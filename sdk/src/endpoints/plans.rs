use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::plan::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Plan, PagBankError> {
    let resp = client
        .post(Service::Recurring, "/plans", body, &RequestOptions::default())
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Plan, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/plans/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Plan>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/plans".to_string()
    } else {
        format!("/plans?{query}")
    };
    let resp = client.get(Service::Recurring, &path).await?;
    crate::models::parse_list(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Plan, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            &format!("/plans/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn activate(client: &PagBankClient, id: &str) -> Result<Plan, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/plans/{id}/activate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn deactivate(client: &PagBankClient, id: &str) -> Result<Plan, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/plans/{id}/deactivate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
