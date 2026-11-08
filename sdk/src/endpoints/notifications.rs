use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::notification::*;

const PATH: &str = "/notifications/preferences";

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<NotificationPreference, PagBankError> {
    let resp = client
        .post(Service::Main, PATH, body, &RequestOptions::default())
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<NotificationPreference, PagBankError> {
    let resp = client.get(Service::Main, &format!("{PATH}/{id}")).await?;
    crate::models::parse_response(resp).await
}

pub async fn list(client: &PagBankClient) -> Result<Vec<NotificationPreference>, PagBankError> {
    let resp = client.get(Service::Main, PATH).await?;
    crate::models::parse_list(resp).await
}

pub async fn update(
    client: &PagBankClient,
    id: &str,
    body: &serde_json::Value,
) -> Result<NotificationPreference, PagBankError> {
    let resp = client
        .patch(
            Service::Main,
            &format!("{PATH}/{id}"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn delete(client: &PagBankClient, id: &str) -> Result<(), PagBankError> {
    let resp = client
        .delete(Service::Main, &format!("{PATH}/{id}"))
        .await?;
    crate::models::parse_void(resp).await
}
