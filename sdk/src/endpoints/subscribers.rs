use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::subscriber::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Subscriber, PagBankError> {
    let resp = client
        .post(
            Service::Recurring,
            "/customers",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Subscriber, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/customers/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Subscriber>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/customers".to_string()
    } else {
        format!("/customers?{query}")
    };
    let resp = client.get(Service::Recurring, &path).await?;
    crate::models::parse_list(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Subscriber, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            &format!("/customers/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn update_payment(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Subscriber, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            &format!("/customers/{id}/payment-method"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
