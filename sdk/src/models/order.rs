use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<OrderCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_codes: Option<Vec<QrCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<super::charge::Charge>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Split>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<SubMerchant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Wallet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<Phone>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMerchant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<OrderCustomer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_codes: Option<Vec<CreateQrCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<super::charge::ChargeRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splits: Option<Split>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQrCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_deserialization() {
        let json = r#"{
            "id": "ORDE_12345",
            "reference_id": "ped-001",
            "created_at": "2024-01-15T10:00:00-03:00",
            "customer": {
                "name": "João",
                "email": "joao@test.com",
                "tax_id": "12345678909"
            },
            "items": [{
                "name": "Produto",
                "quantity": 1,
                "unit_amount": 1000
            }]
        }"#;
        let order: Order = serde_json::from_str(json).unwrap();
        assert_eq!(order.id, Some("ORDE_12345".to_string()));
        assert_eq!(order.reference_id, Some("ped-001".to_string()));
        let customer = order.customer.unwrap();
        assert_eq!(customer.name, Some("João".to_string()));
        assert_eq!(customer.email, Some("joao@test.com".to_string()));
        let items = order.items.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Produto");
    }

    #[test]
    fn order_serialization_omits_none() {
        let order = Order {
            id: Some("ORDE_1".to_string()),
            reference_id: None,
            created_at: None,
            customer: None,
            items: None,
            shipping: None,
            qr_codes: None,
            charges: None,
            notification_urls: None,
            splits: None,
            links: None,
            metadata: None,
            sub_merchant: None,
            wallet: None,
        };
        let json = serde_json::to_value(&order).unwrap();
        assert_eq!(json["id"], "ORDE_1");
        assert!(json.get("reference_id").is_none());
        assert!(json.get("customer").is_none());
    }

    #[test]
    fn create_order_request_serialization() {
        let request = CreateOrderRequest {
            reference_id: Some("ped-001".to_string()),
            customer: Some(OrderCustomer {
                name: Some("Maria".to_string()),
                email: Some("maria@test.com".to_string()),
                tax_id: Some("12345678909".to_string()),
                phones: None,
            }),
            items: Some(vec![Item {
                reference_id: None,
                name: "Produto".to_string(),
                quantity: 1,
                unit_amount: 5000,
            }]),
            shipping: None,
            qr_codes: None,
            notification_urls: None,
            charges: None,
            splits: None,
            metadata: None,
        };
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["reference_id"], "ped-001");
        assert_eq!(json["customer"]["name"], "Maria");
        assert_eq!(json["items"][0]["unit_amount"], 5000);
    }

    #[test]
    fn order_customer_deserialization() {
        let json = r#"{
            "name": "Ana",
            "email": "ana@test.com",
            "tax_id": "98765432100",
            "phones": [{
                "area": "11",
                "number": "988887777",
                "type": "MOBILE"
            }]
        }"#;
        let customer: OrderCustomer = serde_json::from_str(json).unwrap();
        assert_eq!(customer.name, Some("Ana".to_string()));
        let phones = customer.phones.unwrap();
        assert_eq!(phones.len(), 1);
        assert_eq!(phones[0].number, "988887777");
    }
}
