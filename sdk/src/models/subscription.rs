use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles: Option<SubscriptionCycles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<super::charge::PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<super::coupon::Coupon>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCycles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub plan_id: String,
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<super::charge::PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscriptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<super::charge::PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<String>>,
}
