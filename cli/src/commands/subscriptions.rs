use anyhow::Result;

use crate::cli::SubscriptionsAction;
use crate::output;

pub async fn run(
    action: SubscriptionsAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        SubscriptionsAction::Create {
            plan_id,
            subscriber_id,
            start_at,
        } => {
            let mut body = serde_json::json!({
                "plan_id": plan_id,
                "customer_id": subscriber_id,
            });
            if let Some(s) = start_at {
                body["start_at"] = serde_json::json!(s);
            }
            let result = pagbank_sdk::endpoints::subscriptions::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Assinatura Criada", &val)
                }
            }
            Ok(())
        }
        SubscriptionsAction::Get { id } => {
            let result = pagbank_sdk::endpoints::subscriptions::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Assinatura", &val),
            }
            Ok(())
        }
        SubscriptionsAction::List {
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
            let result = pagbank_sdk::endpoints::subscriptions::list(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|s| {
                                vec![
                                    s["id"].as_str().unwrap_or("").to_string(),
                                    s["plan_id"].as_str().unwrap_or("").to_string(),
                                    s["customer_id"].as_str().unwrap_or("").to_string(),
                                    s["status"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Plano", "Assinante", "Status"], rows);
                    }
                }
            }
            Ok(())
        }
        SubscriptionsAction::Update { id, plan_id } => {
            let mut body = serde_json::json!({});
            if let Some(p) = plan_id {
                body["plan_id"] = serde_json::json!(p);
            }
            let result = pagbank_sdk::endpoints::subscriptions::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Assinatura Atualizada", &val)
                }
            }
            Ok(())
        }
        SubscriptionsAction::Cancel { id } => {
            let result = pagbank_sdk::endpoints::subscriptions::cancel(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Assinatura cancelada");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Assinatura", &val),
            }
            Ok(())
        }
        SubscriptionsAction::Suspend { id } => {
            let result = pagbank_sdk::endpoints::subscriptions::suspend(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Assinatura suspensa");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Assinatura", &val),
            }
            Ok(())
        }
        SubscriptionsAction::Activate { id } => {
            let result =
                pagbank_sdk::endpoints::subscriptions::activate_subscription(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Assinatura ativada");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Assinatura", &val),
            }
            Ok(())
        }
        SubscriptionsAction::Invoices { id } => {
            let _params: Vec<(String, String)> = vec![];
            let result = pagbank_sdk::endpoints::invoices::list_payments(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|i| {
                                vec![
                                    i["id"].as_str().unwrap_or("").to_string(),
                                    i["status"].as_str().unwrap_or("").to_string(),
                                    i["amount"]["total"].to_string(),
                                    i["paid_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Status", "Total", "Pago em"], rows);
                    }
                }
            }
            Ok(())
        }
        SubscriptionsAction::DeleteCoupons { id } => {
            pagbank_sdk::endpoints::subscriptions::delete_coupons(&client, &id).await?;
            output::print_success("Cupons removidos da assinatura");
            Ok(())
        }
    }
}
