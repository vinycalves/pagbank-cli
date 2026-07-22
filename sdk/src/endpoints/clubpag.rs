use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::clubpag::*;

pub async fn get_settings(client: &PagBankClient) -> Result<ClubPagSettings, PagBankError> {
    let resp = client.get(Service::Main, "/clubpag/settings").await?;
    crate::models::parse_response(resp).await
}

pub async fn update_settings(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<ClubPagSettings, PagBankError> {
    let resp = client
        .put(
            Service::Main,
            "/clubpag/settings",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn identify_purchase(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/clubpag/purchase",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn list_benefits(client: &PagBankClient) -> Result<Vec<ClubPagBenefit>, PagBankError> {
    let resp = client.get(Service::Main, "/clubpag/benefits").await?;
    crate::models::parse_list(resp).await
}

pub async fn redeem_benefit(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/clubpag/benefits/redeem",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_cashback(client: &PagBankClient) -> Result<ClubPagCashback, PagBankError> {
    let resp = client.get(Service::Main, "/clubpag/cashback").await?;
    crate::models::parse_response(resp).await
}

pub async fn list_coupons(client: &PagBankClient) -> Result<Vec<ClubPagCoupon>, PagBankError> {
    let resp = client.get(Service::Main, "/clubpag/coupons").await?;
    crate::models::parse_list(resp).await
}
