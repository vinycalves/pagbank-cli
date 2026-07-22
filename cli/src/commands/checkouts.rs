use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};

use crate::cli::CheckoutsAction;
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

pub async fn run(action: CheckoutsAction, env_override: Option<&str>, output_fmt: &crate::cli::OutputFormat) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        CheckoutsAction::Create { name, amount, description, redirect_url, payment_methods } => {
            let mut body = serde_json::json!({
                "name": name,
                "amount": { "value": amount, "currency": "BRL" },
            });
            if let Some(d) = description { body["description"] = serde_json::json!(d); }
            if let Some(r) = redirect_url { body["redirect_url"] = serde_json::json!(r); }
            if let Some(pm) = payment_methods {
                let methods: Vec<String> = pm.split(',').map(|s| s.trim().to_string()).collect();
                body["payment_methods"] = serde_json::json!(methods);
            }
            let result = pagbank_sdk::endpoints::checkouts::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Checkout Criado", &val),
            }
            Ok(())
        }
        CheckoutsAction::Get { id } => {
            let result = pagbank_sdk::endpoints::checkouts::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Checkout", &val),
            }
            Ok(())
        }
        CheckoutsAction::Activate { id } => {
            let result = pagbank_sdk::endpoints::checkouts::activate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Checkout ativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Checkout", &val),
            }
            Ok(())
        }
        CheckoutsAction::Deactivate { id } => {
            let result = pagbank_sdk::endpoints::checkouts::deactivate(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Checkout inativado");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Checkout", &val),
            }
            Ok(())
        }
    }
}
