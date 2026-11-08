use serde::{Deserialize, Serialize};

use super::common::{Amount, Link};

/// Chargeback (contestação) recebido pelo vendedor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chargeback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<ChargebackTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargebackTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installment_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_card_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

/// Defesa enviada contra um chargeback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<DisputeFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chargeback_deserialization() {
        let json = r#"{
            "id": "CBKS_3EA3FC40-192C-A6BB-ACA0-B3DAFA537DF6",
            "created_at": "2024-09-30T10:37:55",
            "updated_at": "2024-10-01T14:46:03.004006",
            "due_date": "2024-11-01T14:46:03.004006",
            "amount": {"value": 100, "currency": "BRL"},
            "classification": "DISAGREEMENT",
            "reason_code": "4853",
            "issuer_comment": null,
            "cycle": "1",
            "status": "CLOSED",
            "transaction": {
                "id": "DF499592BFB74B9798826F2DBD915092",
                "date": "2024-09-30T09:42:45",
                "customer_name": "Jose Francisco da Silva",
                "customer_email": "josefs@yahoo.com.br",
                "amount": 100,
                "installments": 1,
                "installment_amount": 100,
                "soft_descriptor": "PAG*RicardoHenriqueRa",
                "masked_card_number": "542820******8829",
                "brand": "MASTERCARD",
                "arn": "123",
                "tid": "C9E790CB608748C6BD9F61CC246141D1",
                "nsu": "093056006098",
                "authorization_code": "471974",
                "reference_id": null
            }
        }"#;
        let cbk: Chargeback = serde_json::from_str(json).unwrap();
        assert_eq!(
            cbk.id.as_deref(),
            Some("CBKS_3EA3FC40-192C-A6BB-ACA0-B3DAFA537DF6")
        );
        assert_eq!(cbk.status.as_deref(), Some("CLOSED"));
        assert_eq!(cbk.amount.as_ref().unwrap().value, 100);
        let tx = cbk.transaction.unwrap();
        assert_eq!(tx.brand.as_deref(), Some("MASTERCARD"));
        assert_eq!(tx.installments, Some(1));
    }

    #[test]
    fn dispute_deserialization() {
        let json = r#"{
            "id": "DISP_C8B27A16-F77B-48AD-A5DD-808400CA15C7",
            "created_at": "2025-02-04T14:13:25",
            "status": "RECEIVED",
            "file": {"type": "application/pdf", "name": "nome_arquivo.pdf"},
            "links": [{
                "rel": "CHARGEBACK",
                "href": "https://api.pagseguro.com/chargebacks/CBKS_A836405B",
                "media": "application/json",
                "type": "GET"
            }]
        }"#;
        let dispute: Dispute = serde_json::from_str(json).unwrap();
        assert_eq!(dispute.status.as_deref(), Some("RECEIVED"));
        assert_eq!(
            dispute.file.as_ref().unwrap().file_type.as_deref(),
            Some("application/pdf")
        );
        assert_eq!(dispute.links.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn dispute_optional_fields() {
        let json = r#"{"id": "DISP_1"}"#;
        let dispute: Dispute = serde_json::from_str(json).unwrap();
        assert!(dispute.file.is_none());
        assert!(dispute.status.is_none());
    }
}
