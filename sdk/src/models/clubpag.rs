use serde::{Deserialize, Serialize};

use super::common::Amount;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagBenefit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagCashback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagCoupon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagPurchaseRequest {
    pub order_id: String,
    pub amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubPagRedeemRequest {
    pub benefit_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateClubPagSettingsRequest {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
