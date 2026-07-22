use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::order::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
    opts: &RequestOptions,
) -> Result<Order, PagBankError> {
    let resp = client.post(Service::Main, "/orders", body, opts).await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Order, PagBankError> {
    let resp = client.get(Service::Main, &format!("/orders/{id}")).await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Order>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/orders".to_string()
    } else {
        format!("/orders?{query}")
    };
    let resp = client.get(Service::Main, &path).await?;
    crate::models::parse_list(resp).await
}

pub async fn pay(
    client: &PagBankClient,
    order_id: &str,
    body: &serde_json::Value,
) -> Result<Order, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            &format!("/orders/{order_id}/pay"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_split(
    client: &PagBankClient,
    order_id: &str,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/orders/{order_id}/splits"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn release_split(client: &PagBankClient, order_id: &str) -> Result<(), PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .post(
            Service::Main,
            &format!("/orders/{order_id}/splits/release"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_void(resp).await
}
