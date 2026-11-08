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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dwo: Option<Dwo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<super::order::SubMerchant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeAmount {
    pub value: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ChargeSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<ChargeFees>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeFees {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer: Option<ChargeBuyerFees>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeBuyerFees {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<ChargeInterest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeInterest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_data: Option<TokenData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<AuthenticationMethod>,
}

/// Dados adicionais de tokenização de rede (Visa/Mastercard).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assurance_level: Option<i32>,
}

/// Dados de autenticação 3DS/InApp vinculados à transação.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub authentication_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstrans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Indica que a cobrança é proveniente de uma recorrência própria do lojista.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurring {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub recurring_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_token: Option<String>,
    #[serde(default, deserialize_with = "de_string_to_i32_opt")]
    pub exp_month: Option<i32>,
    #[serde(default, deserialize_with = "de_string_to_i32_opt")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<CardWallet>,
}

fn de_string_to_i32_opt<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct StringOrInt;
    impl<'de> de::Visitor<'de> for StringOrInt {
        type Value = Option<i32>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("string or integer")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Option<i32>, E> {
            Ok(Some(v as i32))
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Option<i32>, E> {
            Ok(v.parse::<i32>().ok())
        }
    }
    deserializer.deserialize_any(StringOrInt)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardHolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
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
pub struct Dwo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<DwoSubMerchant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<DwoReceiver>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_in: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwoSubMerchant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DwoReceiver {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number_type: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dwo: Option<Dwo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<super::order::SubMerchant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayOrderRequest {
    pub payment_method: PaymentMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStoreRequest {
    pub card: Card,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charge_amount_with_buyer_fees() {
        let json = r#"{
            "value": 10887,
            "currency": "BRL",
            "fees": {
                "buyer": {
                    "interest": {"total": 887, "installments": 4}
                }
            },
            "summary": {"total": 10887, "paid": 0, "refunded": 0}
        }"#;
        let amount: ChargeAmount = serde_json::from_str(json).unwrap();
        let fees = amount.fees.unwrap();
        let interest = fees.buyer.unwrap().interest.unwrap();
        assert_eq!(interest.total, Some(887));
        assert_eq!(interest.installments, Some(4));
    }

    #[test]
    fn charge_request_serializes_sub_merchant_and_recurring() {
        let request = ChargeRequest {
            reference_id: None,
            description: None,
            amount: ChargeAmount {
                value: 1000,
                currency: "BRL".to_string(),
                summary: None,
                fees: None,
            },
            payment_method: None,
            dwo: None,
            sub_merchant: Some(super::super::order::SubMerchant {
                reference_id: Some("SUB-1".to_string()),
                name: Some("Loja".to_string()),
                tax_id: Some("12345678909".to_string()),
                mcc: Some("155".to_string()),
                address: None,
            }),
            notification_urls: Some(vec!["https://meusite.com/webhook".to_string()]),
            recurring: Some(Recurring {
                recurring_type: Some("INITIAL".to_string()),
                recurrence_id: None,
                original_amount: Some(1000),
            }),
        };
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["sub_merchant"]["reference_id"], "SUB-1");
        assert_eq!(json["recurring"]["type"], "INITIAL");
        assert_eq!(json["notification_urls"][0], "https://meusite.com/webhook");
    }

    #[test]
    fn payment_method_token_data_and_authentication() {
        let json = r#"{
            "type": "DEBIT_CARD",
            "token_data": {
                "requestor_id": "12345678901",
                "wallet": "APPLE_PAY",
                "cryptogram": "abc",
                "assurance_level": 99
            },
            "authentication_method": {
                "id": "3DS_15CB7893",
                "type": "THREEDS",
                "eci": "01",
                "status": "AUTHENTICATED"
            }
        }"#;
        let method: PaymentMethod = serde_json::from_str(json).unwrap();
        assert_eq!(
            method.token_data.as_ref().unwrap().wallet.as_deref(),
            Some("APPLE_PAY")
        );
        let auth = method.authentication_method.unwrap();
        assert_eq!(auth.authentication_type.as_deref(), Some("THREEDS"));
        assert_eq!(auth.status.as_deref(), Some("AUTHENTICATED"));
    }
}
