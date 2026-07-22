use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::invoice::*;

pub async fn get(client: &PagBankClient, id: &str) -> Result<Invoice, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/invoices/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list_payments(
    client: &PagBankClient,
    invoice_id: &str,
) -> Result<Vec<InvoicePayment>, PagBankError> {
    let resp = client
        .get(
            Service::Recurring,
            &format!("/invoices/{invoice_id}/payments"),
        )
        .await?;
    crate::models::parse_list(resp).await
}

pub async fn get_payment(
    client: &PagBankClient,
    payment_id: &str,
) -> Result<InvoicePayment, PagBankError> {
    let resp = client
        .get(Service::Recurring, &format!("/payments/{payment_id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn create_refund(
    client: &PagBankClient,
    payment_id: &str,
    body: &serde_json::Value,
) -> Result<Refund, PagBankError> {
    let resp = client
        .post(
            Service::Recurring,
            &format!("/payments/{payment_id}/refund"),
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list_refunds(
    client: &PagBankClient,
    payment_id: &str,
) -> Result<Vec<Refund>, PagBankError> {
    let resp = client
        .get(
            Service::Recurring,
            &format!("/payments/{payment_id}/refunds"),
        )
        .await?;
    crate::models::parse_list(resp).await
}

pub async fn list_all_payments(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<InvoicePayment>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/payments".to_string()
    } else {
        format!("/payments?{query}")
    };
    let resp = client.get(Service::Recurring, &path).await?;
    crate::models::parse_list(resp).await
}
