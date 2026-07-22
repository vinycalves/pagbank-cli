use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::subscription::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Subscription, PagBankError> {
    let resp = client
        .post(
            Service::Recurring,
            "/subscriptions",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Subscription, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/subscriptions/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Subscription>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/subscriptions".to_string()
    } else {
        format!("/subscriptions?{query}")
    };
    let resp = client.get(Service::Recurring, &path).await?;
    crate::models::parse_list(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<Subscription, PagBankError> {
    let resp = client
        .put(
            Service::Recurring,
            &format!("/subscriptions/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn cancel(client: &PagBankClient, id: &str) -> Result<Subscription, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/subscriptions/{id}/cancel"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn suspend(client: &PagBankClient, id: &str) -> Result<Subscription, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/subscriptions/{id}/suspend"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn activate_subscription(
    client: &PagBankClient,
    id: &str,
) -> Result<Subscription, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/subscriptions/{id}/activate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn delete_coupons(client: &PagBankClient, id: &str) -> Result<(), PagBankError> {
    let resp = client
        .delete(Service::Recurring, &format!("/subscriptions/{id}/coupons"))
        .await?;
    crate::models::parse_void(resp).await
}
