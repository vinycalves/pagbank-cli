use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::account::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Account, PagBankError> {
    let resp = client
        .post(Service::Main, "/accounts", body, &RequestOptions::default())
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<Account, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/accounts/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}
