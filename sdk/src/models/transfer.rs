use serde::{Deserialize, Serialize};

use super::common::Link;

/// Transferência P2P (entre contas PagBank) ou PIX (outras instituições).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enToEndId")]
    pub en_to_end_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<TransferAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument: Option<TransferInstrument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAmount {
    /// A API alterna entre número e string neste campo.
    #[serde(
        default,
        deserialize_with = "de_i64_from_string_or_int",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

fn de_i64_from_string_or_int<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct StringOrInt;
    impl<'de> de::Visitor<'de> for StringOrInt {
        type Value = Option<i64>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("string or integer")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Option<i64>, E> {
            Ok(Some(v))
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Option<i64>, E> {
            Ok(Some(v as i64))
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Option<i64>, E> {
            Ok(v.parse::<i64>().ok())
        }
        fn visit_none<E: de::Error>(self) -> Result<Option<i64>, E> {
            Ok(None)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Option<i64>, E> {
            Ok(None)
        }
    }
    deserializer.deserialize_any(StringOrInt)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferInstrument {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub instrument_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p2p: Option<TransferP2p>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<TransferPix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferP2p {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPix {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_ispb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_p2p_deserialization() {
        let json = r#"{
            "id": "TRAN_8630BB43-1136-431E-A187-D2EA616526BE",
            "created_at": "2021-11-08T14:51:28.045-03:00",
            "status": "CREATED",
            "enToEndId": "5c3e369a-babf-445e-b335-fceb7672fc2e",
            "amount": {"value": "099", "currency": "BRL"},
            "description": "valor referente as vendas do dia 01/01",
            "reference_id": "referencia_cliente12345",
            "instrument": {
                "type": "P2P",
                "p2p": {
                    "bank_branch": "1234",
                    "account_number": "12345678",
                    "account_id": "ACCO_5C3BC4E3-A83B-4872-B099-743121BAC985"
                }
            },
            "notification_urls": ["https://meusite.com/notificacoes"]
        }"#;
        let transfer: Transfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.status.as_deref(), Some("CREATED"));
        assert_eq!(transfer.amount.as_ref().unwrap().value, Some(99));
        assert_eq!(
            transfer
                .instrument
                .as_ref()
                .unwrap()
                .instrument_type
                .as_deref(),
            Some("P2P")
        );
        assert_eq!(
            transfer
                .instrument
                .as_ref()
                .unwrap()
                .p2p
                .as_ref()
                .unwrap()
                .bank_branch
                .as_deref(),
            Some("1234")
        );
    }

    #[test]
    fn transfer_pix_amount_as_number() {
        let json = r#"{
            "id": "TRAN_2",
            "amount": {"value": 150099, "currency": "BRL"},
            "instrument": {"type": "PIX", "pix": {"key": "chave@pix.com"}}
        }"#;
        let transfer: Transfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.amount.as_ref().unwrap().value, Some(150099));
        assert_eq!(
            transfer
                .instrument
                .as_ref()
                .unwrap()
                .pix
                .as_ref()
                .unwrap()
                .key
                .as_deref(),
            Some("chave@pix.com")
        );
    }

    #[test]
    fn transfer_without_instrument() {
        let transfer: Transfer = serde_json::from_str(r#"{"id":"TRAN_3"}"#).unwrap();
        assert!(transfer.instrument.is_none());
        assert!(transfer.amount.is_none());
    }
}
