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
    pub payment: Option<RefundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub refund_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<super::common::Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerRefund {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<SellerRefundPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<super::common::Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerRefundPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<InvoiceAmount>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refund_deserialization() {
        let json = r#"{
            "id": "REFU_F5B60BD0-508F-4DA1-B73E-87CFF76478F4",
            "payment": {
                "id": "PAYM_CONTRACT-TEST-TEST-TEST-CONTRACT0006",
                "amount": {"total": 10}
            },
            "amount": {"total": 10},
            "created_at": "2021-05-20T08:59:41.222846-03:00",
            "status": "SUCCESS",
            "type": "FULL",
            "links": [{
                "rel": "SELF",
                "href": "https://sandbox.api.assinaturas.pagseguro.com/refunds/REFU_F5B60BD0",
                "media": "application/json",
                "type": "GET"
            }]
        }"#;
        let refund: Refund = serde_json::from_str(json).unwrap();
        assert_eq!(refund.status.as_deref(), Some("SUCCESS"));
        assert_eq!(refund.refund_type.as_deref(), Some("FULL"));
        assert_eq!(
            refund.payment.as_ref().unwrap().id.as_deref(),
            Some("PAYM_CONTRACT-TEST-TEST-TEST-CONTRACT0006")
        );
        assert_eq!(refund.links.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn invoice_deserialization() {
        let json = r#"{
            "id": "INVO_1",
            "subscription_id": "SUBS_1",
            "status": "PAID",
            "amount": {"total": 4990, "paid": 4990, "refunded": 0},
            "cycle": {"actual": 1, "count": 12}
        }"#;
        let invoice: Invoice = serde_json::from_str(json).unwrap();
        assert_eq!(invoice.status.as_deref(), Some("PAID"));
        assert_eq!(invoice.cycle.unwrap().actual, Some(1));
        assert_eq!(invoice.amount.unwrap().total, Some(4990));
    }
}
