use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};

use crate::cli::InvoicesAction;
use crate::config::PbConfig;
use crate::output;

fn make_client(config: &PbConfig, env_override: Option<&str>) -> Result<PagBankClient> {
    let active = config.get_active_config(env_override);
    let pagbank_config = PagBankConfig {
        environment: active.environment.parse().unwrap_or(Environment::Sandbox),
        token: active.token.clone(),
        recurring_token: active.recurring_token.clone(),
        client_id: active.client_id.clone(),
        client_secret: active.client_secret.clone(),
    };
    Ok(PagBankClient::new(pagbank_config))
}

pub async fn run(action: InvoicesAction, env_override: Option<&str>, output_fmt: &crate::cli::OutputFormat) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        InvoicesAction::Get { id } => {
            let result = pagbank_sdk::endpoints::invoices::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Fatura", &val),
            }
            Ok(())
        }
        InvoicesAction::Payments { invoice_id } => {
            let result = pagbank_sdk::endpoints::invoices::list_payments(&client, &invoice_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr.iter().map(|p| {
                            vec![
                                p["id"].as_str().unwrap_or("").to_string(),
                                p["status"].as_str().unwrap_or("").to_string(),
                                p["amount"]["total"].to_string(),
                                p["paid_at"].as_str().unwrap_or("").to_string(),
                            ]
                        }).collect();
                        output::print_table(&["ID", "Status", "Total", "Pago em"], rows);
                    }
                }
            }
            Ok(())
        }
        InvoicesAction::Refund { payment_id, amount } => {
            let body = if let Some(a) = amount {
                serde_json::json!({ "amount": a })
            } else {
                serde_json::json!({})
            };
            let result = pagbank_sdk::endpoints::invoices::create_refund(&client, &payment_id, &body).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Estorno criado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Estorno", &val),
            }
            Ok(())
        }
        InvoicesAction::ListRefunds { payment_id } => {
            let result = pagbank_sdk::endpoints::invoices::list_refunds(&client, &payment_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr.iter().map(|r| {
                            vec![
                                r["id"].as_str().unwrap_or("").to_string(),
                                r["status"].as_str().unwrap_or("").to_string(),
                                r["amount"]["total"].to_string(),
                                r["created_at"].as_str().unwrap_or("").to_string(),
                            ]
                        }).collect();
                        output::print_table(&["ID", "Status", "Total", "Criado em"], rows);
                    }
                }
            }
            Ok(())
        }
        InvoicesAction::GetPayment { payment_id } => {
            let result = pagbank_sdk::endpoints::invoices::get_payment(&client, &payment_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Pagamento Recorrente", &val),
            }
            Ok(())
        }
    }
}
