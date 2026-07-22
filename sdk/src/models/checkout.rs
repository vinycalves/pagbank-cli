use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
