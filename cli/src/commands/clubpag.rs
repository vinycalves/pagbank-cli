use anyhow::Result;

use crate::cli::ClubPagAction;
use crate::output;

pub async fn run(
    action: ClubPagAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        ClubPagAction::Settings => {
            let result = pagbank_sdk::endpoints::clubpag::get_settings(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Configurações ClubPag", &val)
                }
            }
            Ok(())
        }
        ClubPagAction::UpdateSettings { enabled } => {
            let body = serde_json::json!({ "enabled": enabled });
            let result = pagbank_sdk::endpoints::clubpag::update_settings(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Configurações atualizadas");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Configurações", &val)
                }
            }
            Ok(())
        }
        ClubPagAction::Purchase { order_id, amount } => {
            let body = serde_json::json!({
                "order_id": order_id,
                "amount": { "value": amount, "currency": "BRL" },
            });
            let result = pagbank_sdk::endpoints::clubpag::identify_purchase(&client, &body).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Compra Identificada", &result)
                }
            }
            Ok(())
        }
        ClubPagAction::Benefits => {
            let result = pagbank_sdk::endpoints::clubpag::list_benefits(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|b| {
                                vec![
                                    b["id"].as_str().unwrap_or("").to_string(),
                                    b["name"].as_str().unwrap_or("").to_string(),
                                    b["status"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Nome", "Status"], rows);
                    }
                }
            }
            Ok(())
        }
        ClubPagAction::Redeem { benefit_id, amount } => {
            let mut body = serde_json::json!({ "benefit_id": benefit_id });
            if let Some(a) = amount {
                body["amount"] = serde_json::json!({ "value": a, "currency": "BRL" });
            }
            let result = pagbank_sdk::endpoints::clubpag::redeem_benefit(&client, &body).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Benefício Resgatado", &result)
                }
            }
            Ok(())
        }
        ClubPagAction::Cashback => {
            let result = pagbank_sdk::endpoints::clubpag::get_cashback(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Cashback", &val),
            }
            Ok(())
        }
        ClubPagAction::Coupons => {
            let result = pagbank_sdk::endpoints::clubpag::list_coupons(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|c| {
                                vec![
                                    c["id"].as_str().unwrap_or("").to_string(),
                                    c["name"].as_str().unwrap_or("").to_string(),
                                    c["description"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Nome", "Descrição"], rows);
                    }
                }
            }
            Ok(())
        }
    }
}
