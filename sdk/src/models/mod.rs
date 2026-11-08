use serde::{Deserialize, Serialize};

use crate::error::PagBankError;

pub mod account;
pub mod certificate;
pub mod charge;
pub mod chargeback;
pub mod checkout;
pub mod clubpag;
pub mod common;
pub mod connect;
pub mod coupon;
pub mod edi;
pub mod invoice;
pub mod notification;
pub mod order;
pub mod plan;
pub mod public_key;
pub mod subscriber;
pub mod subscription;
pub mod transfer;

pub use account::*;
pub use charge::*;
pub use chargeback::*;
pub use checkout::*;
pub use clubpag::*;
pub use common::*;
pub use connect::*;
pub use coupon::*;
pub use edi::*;
pub use invoice::*;
pub use notification::*;
pub use order::*;
pub use plan::*;
pub use public_key::*;
pub use subscriber::*;
pub use subscription::*;
pub use transfer::*;

pub async fn parse_response<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<T, PagBankError> {
    let json: serde_json::Value = resp.json().await?;
    Ok(serde_json::from_value(json)?)
}

pub async fn parse_list<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<Vec<T>, PagBankError> {
    let json: serde_json::Value = resp.json().await?;
    for key in &[
        "data",
        "orders",
        "items",
        "payments",
        "refunds",
        "preferences",
        "transfers",
    ] {
        if let Some(arr) = json.get(*key).and_then(|v| v.as_array()) {
            let items: Vec<T> = arr
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect();
            return Ok(items);
        }
    }
    if let Some(arr) = json.as_array() {
        let items: Vec<T> = arr
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        return Ok(items);
    }
    Ok(vec![])
}

pub async fn parse_void(resp: reqwest::Response) -> Result<(), PagBankError> {
    if resp.status().is_success() {
        Ok(())
    } else {
        let status = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Err(PagBankError::ApiRaw { status, body })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Option<Vec<T>>,
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}
