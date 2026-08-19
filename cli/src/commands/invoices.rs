use anyhow::Result;

use crate::cli::InvoicesAction;
use crate::output;

pub async fn run(
    action: InvoicesAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

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
            let result =
                pagbank_sdk::endpoints::invoices::list_payments(&client, &invoice_id).await?;
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
                                    p["status"].as_str().unwrap_or("").to_string(),
                                    p["amount"]["total"].to_string(),
                                    p["paid_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
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
            let result =
                pagbank_sdk::endpoints::invoices::create_refund(&client, &payment_id, &body)
                    .await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Estorno criado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Estorno", &val),
            }
            Ok(())
        }
        InvoicesAction::ListRefunds { payment_id } => {
            let result =
                pagbank_sdk::endpoints::invoices::list_refunds(&client, &payment_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|r| {
                                vec![
                                    r["id"].as_str().unwrap_or("").to_string(),
                                    r["status"].as_str().unwrap_or("").to_string(),
                                    r["amount"]["total"].to_string(),
                                    r["created_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Status", "Total", "Criado em"], rows);
                    }
                }
            }
            Ok(())
        }
        InvoicesAction::GetPayment { payment_id } => {
            let result =
                pagbank_sdk::endpoints::invoices::get_payment(&client, &payment_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Pagamento Recorrente", &val)
                }
            }
            Ok(())
        }
        InvoicesAction::ListPayments {
            status,
            offset,
            limit,
            created_at_start,
            created_at_end,
            payment_method,
        } => {
            let mut params: Vec<(String, String)> = vec![
                ("offset".to_string(), offset.to_string()),
                ("limit".to_string(), limit.to_string()),
            ];
            if let Some(s) = status {
                params.push(("status".to_string(), s));
            }
            if let Some(s) = created_at_start {
                params.push(("created_at_start".to_string(), s));
            }
            if let Some(s) = created_at_end {
                params.push(("created_at_end".to_string(), s));
            }
            if let Some(s) = payment_method {
                params.push(("payment_method_type".to_string(), s));
            }
            let result =
                pagbank_sdk::endpoints::invoices::list_all_payments(&client, &params).await?;
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
                                    p["status"].as_str().unwrap_or("").to_string(),
                                    p["amount"]["total"].to_string(),
                                    p["created_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Status", "Total", "Criado em"], rows);
                    }
                }
            }
            Ok(())
        }
        InvoicesAction::ListSellerRefunds { offset, limit } => {
            let params: Vec<(String, String)> = vec![
                ("offset".to_string(), offset.to_string()),
                ("limit".to_string(), limit.to_string()),
            ];
            let result =
                pagbank_sdk::endpoints::invoices::list_seller_refunds(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|r| {
                                vec![
                                    r["id"].as_str().unwrap_or("").to_string(),
                                    r["status"].as_str().unwrap_or("").to_string(),
                                    r["type"].as_str().unwrap_or("").to_string(),
                                    r["created_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Status", "Tipo", "Criado em"], rows);
                    }
                }
            }
            Ok(())
        }
    }
}
