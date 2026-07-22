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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_serialization() {
        let amount = Amount {
            value: 1000,
            currency: "BRL".to_string(),
        };
        let json = serde_json::to_value(&amount).unwrap();
        assert_eq!(json["value"], 1000);
        assert_eq!(json["currency"], "BRL");
    }

    #[test]
    fn amount_default_currency() {
        let json = r#"{"value": 500}"#;
        let amount: Amount = serde_json::from_str(json).unwrap();
        assert_eq!(amount.currency, "BRL");
    }

    #[test]
    fn amount_deserialization() {
        let json = r#"{"value": 2500, "currency": "USD"}"#;
        let amount: Amount = serde_json::from_str(json).unwrap();
        assert_eq!(amount.value, 2500);
        assert_eq!(amount.currency, "USD");
    }

    #[test]
    fn address_serialization() {
        let address = Address {
            street: Some("Rua Exemplo".to_string()),
            number: Some("123".to_string()),
            complement: None,
            locality: None,
            city: Some("São Paulo".to_string()),
            region: Some("SP".to_string()),
            region_code: Some("SP".to_string()),
            country: "BRA".to_string(),
            postal_code: "01452002".to_string(),
        };
        let json = serde_json::to_value(&address).unwrap();
        assert_eq!(json["street"], "Rua Exemplo");
        assert_eq!(json["number"], "123");
        assert_eq!(json["postal_code"], "01452002");
        assert_eq!(json["country"], "BRA");
        assert!(json.get("complement").is_none());
    }

    #[test]
    fn address_default_country() {
        let json = r#"{"postal_code": "01452002"}"#;
        let address: Address = serde_json::from_str(json).unwrap();
        assert_eq!(address.country, "BRA");
    }

    #[test]
    fn phone_serialization() {
        let phone = Phone {
            country: Some("55".to_string()),
            area: "11".to_string(),
            number: "999999999".to_string(),
            phone_type: Some("MOBILE".to_string()),
        };
        let json = serde_json::to_value(&phone).unwrap();
        assert_eq!(json["area"], "11");
        assert_eq!(json["number"], "999999999");
        assert_eq!(json["type"], "MOBILE");
    }

    #[test]
    fn phone_optional_fields() {
        let json = r#"{"area": "11", "number": "999999999"}"#;
        let phone: Phone = serde_json::from_str(json).unwrap();
        assert!(phone.country.is_none());
        assert!(phone.phone_type.is_none());
    }

    #[test]
    fn item_serialization() {
        let item = Item {
            reference_id: Some("item-001".to_string()),
            name: "Produto".to_string(),
            quantity: 2,
            unit_amount: 1000,
        };
        let json = serde_json::to_value(&item).unwrap();
        assert_eq!(json["name"], "Produto");
        assert_eq!(json["quantity"], 2);
        assert_eq!(json["unit_amount"], 1000);
    }

    #[test]
    fn split_serialization() {
        let split = Split {
            method: "FIXED".to_string(),
            receivers: Some(vec![SplitReceiver {
                amount: SplitAmount { value: 500 },
                account: SplitAccount {
                    id: "ACC_123".to_string(),
                },
                configurations: None,
                reason: None,
            }]),
        };
        let json = serde_json::to_value(&split).unwrap();
        assert_eq!(json["method"], "FIXED");
        assert_eq!(json["receivers"][0]["amount"]["value"], 500);
        assert_eq!(json["receivers"][0]["account"]["id"], "ACC_123");
    }

    #[test]
    fn link_serialization() {
        let link = Link {
            rel: "self".to_string(),
            href: "https://api.pagseguro.com/orders/123".to_string(),
            media: "application/json".to_string(),
            link_type: "GET".to_string(),
        };
        let json = serde_json::to_value(&link).unwrap();
        assert_eq!(json["rel"], "self");
        assert_eq!(json["href"], "https://api.pagseguro.com/orders/123");
    }

    #[test]
    fn link_default_media() {
        let json = r#"{"rel": "self", "href": "https://example.com", "type": "GET"}"#;
        let link: Link = serde_json::from_str(json).unwrap();
        assert_eq!(link.media, "application/json");
    }
}
