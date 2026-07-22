use serde::{Deserialize, Serialize};

use super::charge::CardHolder;
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<SubscriberPaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberPaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<SubscriberCreditCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<SubscriberBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<SubscriberPix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberCreditCard {
    pub number: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub security_code: String,
    pub holder: CardHolder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberBoleto {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberPix {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub name: String,
    pub email: String,
    pub tax_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<SubscriberPaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriberPaymentRequest {
    pub payment_method: SubscriberPaymentMethod,
}
