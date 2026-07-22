use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::certificate::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<Certificate, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/certificates",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
