use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::connect::*;

pub async fn create_app(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<ConnectApp, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/oauth2/application",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_app(client: &PagBankClient, id: &str) -> Result<ConnectApp, PagBankError> {
    let resp = client
        .get(Service::Main, &format!("/oauth2/application/{id}"))
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn get_authorize_url(
    _client: &PagBankClient,
    app_id: &str,
    redirect_uri: &str,
    scope: &str,
) -> String {
    format!(
        "https://sso.pagseguro.uol.com.br/oauth2/authorize?\
         response_type=code&client_id={app_id}&redirect_uri={redirect_uri}&scope={scope}"
    )
}

pub async fn create_sms_auth(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<serde_json::Value, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/oauth2/authorize/sms",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn create_token(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<ConnectToken, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/oauth2/token",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn refresh_token(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<ConnectToken, PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/oauth2/token/refresh",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_response(resp).await
}

pub async fn revoke_token(
    client: &PagBankClient,
    body: &serde_json::Value,
) -> Result<(), PagBankError> {
    let resp = client
        .post(
            Service::Main,
            "/oauth2/token/revoke",
            body,
            &RequestOptions::default(),
        )
        .await?;
    crate::models::parse_void(resp).await
}
