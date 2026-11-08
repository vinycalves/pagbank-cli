use serde::{Deserialize, Serialize};

/// Preferência de notificação da API de Notificação (webhooks).
///
/// Eventos disponíveis: `CHARGEBACK`, `ENROLLMENT`, `CANCELLATION`, `ORDER.CHARGE`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_preference_deserialization() {
        let json = r#"{
            "id": "PREF_B5A9B12B-C852-48AF-9E30-5F42AEC256FB",
            "events": ["CHARGEBACK", "ORDER.CHARGE"],
            "target": "https://meusite.com/notificacoes",
            "created_at": "2024-10-15T16:38:43.7287856-03:00",
            "updated_at": "2024-10-15T16:38:43.7287857-03:00"
        }"#;
        let pref: NotificationPreference = serde_json::from_str(json).unwrap();
        assert_eq!(pref.events.as_ref().unwrap().len(), 2);
        assert_eq!(
            pref.target.as_deref(),
            Some("https://meusite.com/notificacoes")
        );
        assert!(pref.id.is_some());
    }

    #[test]
    fn notification_preference_optional_fields() {
        let pref: NotificationPreference = serde_json::from_str(r#"{"id":"PREF_1"}"#).unwrap();
        assert!(pref.events.is_none());
        assert!(pref.target.is_none());
    }
}
