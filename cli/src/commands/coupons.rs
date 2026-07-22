use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::CouponsAction;
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
    action: CouponsAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        CouponsAction::Create {
            name,
            discount_type,
            discount_value,
            description,
            reference_id,
            limit,
        } => {
            let mut body = serde_json::json!({
                "name": name,
                "discount": {
                    "type": discount_type.to_uppercase(),
                    "value": discount_value,
                },
            });
            if let Some(d) = description {
                body["description"] = serde_json::json!(d);
            }
            if let Some(ri) = reference_id {
                body["reference_id"] = serde_json::json!(ri);
            }
            if let Some(l) = limit {
                body["limit"] = serde_json::json!(l);
            }
            let result = pagbank_sdk::endpoints::coupons::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Cupom Criado", &val),
            }
            Ok(())
        }
        CouponsAction::Get { id } => {
            let result = pagbank_sdk::endpoints::coupons::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Cupom", &val),
            }
            Ok(())
        }
        CouponsAction::List { page, per_page } => {
            let params = vec![
                ("page".to_string(), page.to_string()),
                ("per_page".to_string(), per_page.to_string()),
            ];
            let result = pagbank_sdk::endpoints::coupons::list(&client, &params).await?;
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
                                    c["status"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Nome", "Status"], rows);
                    }
                }
            }
            Ok(())
        }
        CouponsAction::Activate { id } => {
            let result = pagbank_sdk::endpoints::coupons::activate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Cupom ativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Cupom", &val),
            }
            Ok(())
        }
        CouponsAction::Deactivate { id } => {
            let result = pagbank_sdk::endpoints::coupons::deactivate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Cupom inativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Cupom", &val),
            }
            Ok(())
        }
    }
}
