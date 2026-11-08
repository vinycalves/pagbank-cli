use anyhow::Result;

use crate::cli::EdiAction;
use crate::output;

fn field(value: &serde_json::Value, key: &str) -> String {
    match value.get(key) {
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

pub async fn run(
    action: EdiAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    match action {
        EdiAction::Movements {
            r#type,
            date,
            page,
            page_size,
        } => {
            let client = crate::client::load_client(env_override)?;
            let result = pagbank_sdk::endpoints::edi::movements(
                &client,
                r#type.as_str(),
                &date,
                page,
                page_size,
            )
            .await?;
            let val = serde_json::to_value(&result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    let rows: Vec<Vec<String>> = result
                        .detalhes
                        .iter()
                        .map(|d| {
                            vec![
                                field(d, "estabelecimento"),
                                field(d, "codigo_transacao"),
                                field(d, "tipo_evento"),
                                field(d, "valor_total_transacao"),
                                field(d, "data_venda_ajuste"),
                            ]
                        })
                        .collect();
                    output::print_table(
                        &["Estabelecimento", "Transação", "Evento", "Valor", "Data"],
                        rows,
                    );
                    if let Some(pagination) = &result.pagination {
                        output::print_info(&format!(
                            "Página {} de {} — {} itens no total",
                            pagination.page.unwrap_or(1),
                            pagination.total_pages.unwrap_or(1),
                            pagination.total_elements.unwrap_or(0)
                        ));
                    }
                    if result.validado == Some(false) {
                        output::print_info(
                            "Dados ainda em processamento (VALIDADO=FALSE); aguarde o fechamento do dia.",
                        );
                    }
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_extracts_values() {
        let value = serde_json::json!({
            "estabelecimento": "126984538",
            "valor_total_transacao": 108.96,
            "tipo_evento": 1,
            "nulo": null
        });
        assert_eq!(field(&value, "estabelecimento"), "126984538");
        assert_eq!(field(&value, "valor_total_transacao"), "108.96");
        assert_eq!(field(&value, "tipo_evento"), "1");
        assert_eq!(field(&value, "nulo"), "");
        assert_eq!(field(&value, "inexistente"), "");
    }
}
