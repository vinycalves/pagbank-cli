use anyhow::{bail, Result};

use crate::cli::NotificationsAction;
use crate::output;

pub fn build_body(events: &[String], target: Option<&str>) -> serde_json::Value {
    let mut body = serde_json::Map::new();
    if !events.is_empty() {
        body.insert(
            "events".to_string(),
            serde_json::Value::Array(
                events
                    .iter()
                    .map(|e| serde_json::Value::String(e.clone()))
                    .collect(),
            ),
        );
    }
    if let Some(target) = target {
        body.insert(
            "target".to_string(),
            serde_json::Value::String(target.to_string()),
        );
    }
    serde_json::Value::Object(body)
}

pub async fn run(
    action: NotificationsAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        NotificationsAction::Create { events, target } => {
            if events.is_empty() {
                bail!("informe ao menos um evento com --events");
            }
            let body = build_body(&events, Some(&target));
            let result = pagbank_sdk::endpoints::notifications::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Preferência Criada", &val)
                }
            }
            Ok(())
        }
        NotificationsAction::Get { id } => {
            let result = pagbank_sdk::endpoints::notifications::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Preferência de Notificação", &val)
                }
            }
            Ok(())
        }
        NotificationsAction::List => {
            let result = pagbank_sdk::endpoints::notifications::list(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|p| {
                                vec![
                                    p["id"].as_str().unwrap_or("").to_string(),
                                    p["events"]
                                        .as_array()
                                        .map(|e| {
                                            e.iter()
                                                .filter_map(|v| v.as_str())
                                                .collect::<Vec<_>>()
                                                .join(", ")
                                        })
                                        .unwrap_or_default(),
                                    p["target"].as_str().unwrap_or("").to_string(),
                                    p["updated_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Eventos", "Destino", "Atualizado em"], rows);
                    }
                }
            }
            Ok(())
        }
        NotificationsAction::Update { id, events, target } => {
            if events.is_empty() && target.is_none() {
                bail!("informe --events e/ou --target para alterar a preferência");
            }
            let body = build_body(&events, target.as_deref());
            let result = pagbank_sdk::endpoints::notifications::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Preferência Atualizada", &val)
                }
            }
            Ok(())
        }
        NotificationsAction::Delete { id } => {
            pagbank_sdk::endpoints::notifications::delete(&client, &id).await?;
            output::print_success("Preferência excluída");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_body_includes_events_and_target() {
        let events = vec!["CHARGEBACK".to_string(), "ORDER.CHARGE".to_string()];
        let body = build_body(&events, Some("https://meusite.com/hook"));
        assert_eq!(body["events"][0], "CHARGEBACK");
        assert_eq!(body["events"][1], "ORDER.CHARGE");
        assert_eq!(body["target"], "https://meusite.com/hook");
    }

    #[test]
    fn build_body_update_only_target() {
        let body = build_body(&[], Some("https://novo.com/hook"));
        assert!(body.get("events").is_none());
        assert_eq!(body["target"], "https://novo.com/hook");
    }
}
