use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<InvoiceCycle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<super::charge::PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_billing_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceCycle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<super::charge::PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRefundRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
