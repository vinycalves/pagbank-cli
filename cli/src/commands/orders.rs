use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::OrdersAction;
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

pub async fn run(
    action: OrdersAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        OrdersAction::Create {
            reference_id,
            customer_name,
            customer_email,
            customer_tax_id,
            item,
            item_qty,
            item_amount,
            method,
            card_number,
            card_exp_month,
            card_exp_year,
            card_cvv,
            card_holder_name,
            card_holder_tax_id,
            installments,
            notification_url,
            qr_amount,
        } => {
            if let Err(msg) = crate::errors::validate_order_create(&method, qr_amount) {
                anyhow::bail!(msg);
            }

            let charge = match method.to_lowercase().as_str() {
                "pix" => {
                    serde_json::json!({
                        "amount": { "value": item_amount, "currency": "BRL" },
                        "payment_method": {
                            "type": "PIX",
                            "pix": {
                                "holder": { "name": &customer_name, "tax_id": &customer_tax_id },
                                "expiration_date": "2026-12-31T23:59:59-03:00"
                            }
                        },
                    })
                }
                "credit_card" | "debit_card" => {
                    let mut payment_method = serde_json::json!({ "type": method.to_uppercase() });
                    let card = serde_json::json!({
                        "number": card_number.unwrap_or_default(),
                        "exp_month": card_exp_month.unwrap_or(0),
                        "exp_year": card_exp_year.unwrap_or(0),
                        "security_code": card_cvv.unwrap_or_default(),
                        "holder": {
                            "name": card_holder_name.unwrap_or_default(),
                            "tax_id": card_holder_tax_id.unwrap_or_default(),
                        },
                    });
                    payment_method["card"] = card;
                    if method.to_lowercase() == "credit_card" {
                        payment_method["capture"] = serde_json::json!(true);
                        if let Some(inst) = installments {
                            payment_method["installments"] = serde_json::json!(inst);
                        }
                    }
                    serde_json::json!({
                        "amount": { "value": item_amount, "currency": "BRL" },
                        "payment_method": payment_method,
                    })
                }
                _ => anyhow::bail!("método de pagamento inválido: {method}"),
            };
            let charges = vec![charge];

            let mut notification_urls = Vec::new();
            if let Some(url) = notification_url {
                notification_urls.push(url);
            }

            let mut body = serde_json::json!({
                "reference_id": reference_id,
                "customer": {
                    "name": customer_name,
                    "email": customer_email,
                    "tax_id": customer_tax_id,
                },
                "items": [{
                    "name": item,
                    "quantity": item_qty,
                    "unit_amount": item_amount,
                }],
                "charges": charges,
                "notification_urls": if notification_urls.is_empty() { serde_json::json!([]) } else { serde_json::json!(notification_urls) },
            });

            if let Some(qa) = qr_amount {
                if method.to_lowercase() != "pix" {
                    body["qr_codes"] = serde_json::json!([{
                        "amount": { "value": qa, "currency": "BRL" }
                    }]);
                }
            }

            let result =
                pagbank_sdk::endpoints::orders::create(&client, &body, &Default::default()).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Pedido Criado", &val)
                }
            }
            Ok(())
        }
        OrdersAction::Get { id } => {
            let result = pagbank_sdk::endpoints::orders::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Pedido", &val),
            }
            Ok(())
        }
        OrdersAction::List {
            status,
            page,
            per_page,
        } => {
            let mut params = vec![
                ("page".to_string(), page.to_string()),
                ("per_page".to_string(), per_page.to_string()),
            ];
            if let Some(s) = status {
                params.push(("status".to_string(), s));
            }
            let result = pagbank_sdk::endpoints::orders::list(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|o| {
                                vec![
                                    o["id"].as_str().unwrap_or("").to_string(),
                                    o["reference_id"].as_str().unwrap_or("").to_string(),
                                    o["charges"][0]["status"].as_str().unwrap_or("").to_string(),
                                    o["charges"][0]["amount"]["value"].to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Referência", "Status", "Valor"], rows);
                    }
                }
            }
            Ok(())
        }
        OrdersAction::Pay {
            order_id,
            method,
            card_number,
            card_exp_month,
            card_exp_year,
            card_cvv,
            card_holder_name,
            card_holder_tax_id,
            installments,
            card_id,
        } => {
            let mut payment_method = serde_json::json!({ "type": method.to_uppercase() });

            match method.to_lowercase().as_str() {
                "credit_card" | "debit_card" => {
                    if let Some(id) = card_id {
                        payment_method["card"] = serde_json::json!({ "id": id });
                    } else {
                        payment_method["card"] = serde_json::json!({
                            "number": card_number.unwrap_or_default(),
                            "exp_month": card_exp_month.unwrap_or(0),
                            "exp_year": card_exp_year.unwrap_or(0),
                            "security_code": card_cvv.unwrap_or_default(),
                            "holder": {
                                "name": card_holder_name.unwrap_or_default(),
                                "tax_id": card_holder_tax_id.unwrap_or_default(),
                            },
                        });
                    }
                    if method.to_lowercase() == "credit_card" {
                        payment_method["capture"] = serde_json::json!(true);
                        if let Some(inst) = installments {
                            payment_method["installments"] = serde_json::json!(inst);
                        }
                    }
                }
                _ => {}
            }

            let body = serde_json::json!({ "payment_method": payment_method });
            let result = pagbank_sdk::endpoints::orders::pay(&client, &order_id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Pedido Pago", &val),
            }
            Ok(())
        }
        OrdersAction::Capture { charge_id } => {
            let result =
                pagbank_sdk::endpoints::charges::capture(&client, &charge_id, None).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Pagamento Capturado", &val)
                }
            }
            Ok(())
        }
        OrdersAction::Cancel { charge_id } => {
            let result = pagbank_sdk::endpoints::charges::cancel(&client, &charge_id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Pagamento Cancelado", &val)
                }
            }
            Ok(())
        }
        OrdersAction::Split { order_id } => {
            let result = pagbank_sdk::endpoints::orders::get_split(&client, &order_id).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Divisão do Pagamento", &result)
                }
            }
            Ok(())
        }
        OrdersAction::SplitRelease { order_id } => {
            pagbank_sdk::endpoints::orders::release_split(&client, &order_id).await?;
            output::print_success("Divisão liberada com sucesso");
            Ok(())
        }
        OrdersAction::Fees { charge_id } => {
            let result = pagbank_sdk::endpoints::charges::get_costs(&client, &charge_id).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Taxas da Transação", &result)
                }
            }
            Ok(())
        }
        OrdersAction::CardStore {
            number,
            exp_month,
            exp_year,
            security_code,
            holder_name,
            holder_tax_id,
        } => {
            let body = serde_json::json!({
                "card": {
                    "number": number,
                    "exp_month": exp_month,
                    "exp_year": exp_year,
                    "security_code": security_code,
                    "holder": {
                        "name": holder_name,
                        "tax_id": holder_tax_id,
                    },
                    "store": true,
                }
            });
            let result = pagbank_sdk::endpoints::charges::store_card(&client, &body).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Cartão Armazenado", &result)
                }
            }
            Ok(())
        }
    }
}
