use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::chargeback::*;

pub async fn get(client: &PagBankClient, id: &str) -> Result<Chargeback, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/chargebacks/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_dispute(
    client: &PagBankClient,
    chargeback_id: &str,
    dispute_id: &str,
) -> Result<Dispute, PagBankError> {
    let resp = client
        .get(
            Service::Main,
            &format!("/chargebacks/{chargeback_id}/disputes/{dispute_id}"),
        )
        .await?;
    crate::models::parse_response(resp).await
}

/// Envia um PDF (máx. 4 MB e 18 páginas) como evidência de defesa do chargeback.
pub async fn create_dispute(
    client: &PagBankClient,
    chargeback_id: &str,
    file_name: &str,
    file_bytes: Vec<u8>,
) -> Result<Dispute, PagBankError> {
    let part = reqwest::multipart::Part::bytes(file_bytes)
        .file_name(file_name.to_string())
        .mime_str("application/pdf")?;
    let form = reqwest::multipart::Form::new().part("file", part);
    let resp = client
        .post_multipart(
            Service::Main,
            &format!("/chargebacks/{chargeback_id}/disputes"),
            form,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}
