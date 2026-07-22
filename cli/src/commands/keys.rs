use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};

use crate::cli::KeysAction;
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

pub async fn run(action: KeysAction, env_override: Option<&str>, output_fmt: &crate::cli::OutputFormat) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        KeysAction::Create => {
            let result = pagbank_sdk::endpoints::public_keys::create(&client).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Chave Pública Criada", &val),
            }
            Ok(())
        }
        KeysAction::Get { id } => {
            let result = pagbank_sdk::endpoints::public_keys::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Chave Pública", &val),
            }
            Ok(())
        }
        KeysAction::Update { id } => {
            let body = serde_json::json!({});
            let result = pagbank_sdk::endpoints::public_keys::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Chave Pública Atualizada", &val),
            }
            Ok(())
        }
    }
}
