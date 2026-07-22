use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub value: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
}

pub fn default_currency() -> String {
    "BRL".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountSummary {
    pub value: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    pub summary: Option<ChargeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeSummary {
    pub total: Option<i64>,
    pub paid: Option<i64>,
    pub refunded: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(default = "default_country")]
    pub country: String,
    pub postal_code: String,
}

fn default_country() -> String {
    "BRA".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phone {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    pub area: String,
    pub number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub phone_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    pub name: String,
    pub quantity: i32,
    pub unit_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub href: String,
    #[serde(default = "default_media")]
    pub media: String,
    #[serde(rename = "type")]
    pub link_type: String,
}

fn default_media() -> String {
    "application/json".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Split {
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receivers: Option<Vec<SplitReceiver>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitReceiver {
    pub amount: SplitAmount,
    pub account: SplitAccount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<SplitConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitAmount {
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitAccount {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custody: Option<SplitCustody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback: Option<SplitChargeback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitCustody {
    pub apply: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitChargeback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_transfer: Option<SplitChargeTransfer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitChargeTransfer {
    pub percentage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLinks {
    pub rel: String,
    pub href: String,
    #[serde(rename = "type")]
    pub link_type: String,
}
