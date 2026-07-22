use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<ChargeAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ChargeSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_response: Option<PaymentResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_codes: Option<Vec<QrCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Split>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instructions: Option<PaymentInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<RawData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeAmount {
    pub value: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ChargeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_reference_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_digits: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<CardHolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardHolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boleto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_lines: Option<BoletoInstructionLines>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<BoletoHolder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_expiration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoletoInstructionLines {
    pub line_1: String,
    pub line_2: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoletoHolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pix {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<PixHolder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixHolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine: Option<Fine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<Interest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Discount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_advice_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub amount: ChargeAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayOrderRequest {
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStoreRequest {
    pub card: Card,
}
