use serde::{Deserialize, Serialize};

/// Resposta da API EDI (extrato do dia).
///
/// Os campos de cada movimento variam conforme o tipo consultado
/// (`transactional`, `financial`, `cashouts`, `balances`), por isso são
/// preservados como JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiMovements {
    #[serde(default)]
    pub detalhes: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<EdiPagination>,
    /// Indica se os dados do dia já foram totalmente processados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validado: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiPagination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalPages")]
    pub total_pages: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalElements")]
    pub total_elements: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edi_movements_deserialization() {
        let json = r#"{
            "detalhes": [{
                "tipo_registro": "1",
                "estabelecimento": "126984538",
                "tipo_evento": "1",
                "codigo_transacao": "795D7A8F4D4649EF90A8DAA0311316F1",
                "valor_total_transacao": 108.96
            }],
            "pagination": {
                "elements": 1000,
                "totalPages": 1,
                "page": 1,
                "totalElements": 1
            }
        }"#;
        let movements: EdiMovements = serde_json::from_str(json).unwrap();
        assert_eq!(movements.detalhes.len(), 1);
        assert_eq!(movements.detalhes[0]["estabelecimento"], "126984538");
        let pagination = movements.pagination.unwrap();
        assert_eq!(pagination.total_pages, Some(1));
        assert_eq!(pagination.total_elements, Some(1));
    }

    #[test]
    fn edi_movements_empty() {
        let movements: EdiMovements = serde_json::from_str("{}").unwrap();
        assert!(movements.detalhes.is_empty());
        assert!(movements.pagination.is_none());
    }
}
