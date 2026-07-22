use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::coupon::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Coupon, PagBankError> {
    let resp = client
        .post(
            Service::Recurring,
            "/coupons",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Coupon, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/coupons/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<Coupon>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/coupons".to_string()
    } else {
        format!("/coupons?{query}")
    };
    let resp = client.get(Service::Recurring, &path).await?;
    crate::models::parse_list(resp).await
}

pub async fn activate(client: &PagBankClient, id: &str) -> Result<Coupon, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/coupons/{id}/activate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn deactivate(client: &PagBankClient, id: &str) -> Result<Coupon, PagBankError> {
    let body = serde_json::json!({});
    let resp = client
        .put(
            Service::Recurring,
            &format!("/coupons/{id}/deactivate"),
            &body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
