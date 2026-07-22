use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::PlansAction;
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
    action: PlansAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        PlansAction::Create {
            name,
            amount,
            period,
            description,
            reference_id,
            setup_fee,
            billing_cycles,
            trial_length,
            trial_unit,
        } => {
            let unit = match period.to_lowercase().as_str() {
                "day" | "daily" => "DAY",
                "week" | "weekly" => "WEEK",
                "month" | "monthly" => "MONTH",
                "year" | "yearly" | "annual" => "YEAR",
                _ => "MONTH",
            };

            let mut body = serde_json::json!({
                "name": name,
                "amount": { "value": amount, "currency": "BRL" },
                "interval": { "length": 1, "unit": unit },
            });
            if let Some(d) = description {
                body["description"] = serde_json::json!(d);
            }
            if let Some(ri) = reference_id {
                body["reference_id"] = serde_json::json!(ri);
            }
            if let Some(sf) = setup_fee {
                body["setup_fee"] = serde_json::json!(sf);
            }
            if let Some(bc) = billing_cycles {
                body["billing_cycles"] = serde_json::json!(bc);
            }
            if let Some(tl) = trial_length {
                let tu = trial_unit.unwrap_or_else(|| "MONTH".to_string());
                body["trial"] = serde_json::json!({ "length": tl, "unit": tu });
            }

            let result = pagbank_sdk::endpoints::plans::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Plano Criado", &val),
            }
            Ok(())
        }
        PlansAction::Get { id } => {
            let result = pagbank_sdk::endpoints::plans::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Plano", &val),
            }
            Ok(())
        }
        PlansAction::List { page, per_page } => {
            let params = vec![
                ("page".to_string(), page.to_string()),
                ("per_page".to_string(), per_page.to_string()),
            ];
            let result = pagbank_sdk::endpoints::plans::list(&client, &params).await?;
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
                                    p["name"].as_str().unwrap_or("").to_string(),
                                    p["amount"]["value"].to_string(),
                                    p["status"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Nome", "Valor", "Status"], rows);
                    }
                }
            }
            Ok(())
        }
        PlansAction::Update {
            id,
            name,
            amount,
            description,
        } => {
            let mut body = serde_json::json!({});
            if let Some(n) = name {
                body["name"] = serde_json::json!(n);
            }
            if let Some(a) = amount {
                body["amount"] = serde_json::json!({ "value": a, "currency": "BRL" });
            }
            if let Some(d) = description {
                body["description"] = serde_json::json!(d);
            }
            let result = pagbank_sdk::endpoints::plans::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Plano Atualizado", &val)
                }
            }
            Ok(())
        }
        PlansAction::Activate { id } => {
            let result = pagbank_sdk::endpoints::plans::activate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Plano ativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Plano", &val),
            }
            Ok(())
        }
        PlansAction::Deactivate { id } => {
            let result = pagbank_sdk::endpoints::plans::deactivate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Plano inativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Plano", &val),
            }
            Ok(())
        }
    }
}
