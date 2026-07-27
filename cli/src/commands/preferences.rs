use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::PreferencesAction;
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
    action: PreferencesAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        PreferencesAction::Get => {
            let result = pagbank_sdk::endpoints::preferences::get_preferences(&client).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Preferências de Notificação", &result)
                }
            }
            Ok(())
        }
        PreferencesAction::Update { body } => {
            let body_val: serde_json::Value = serde_json::from_str(&body)?;
            let result =
                pagbank_sdk::endpoints::preferences::update_preferences(&client, &body_val).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Preferências Atualizadas", &result)
                }
            }
            Ok(())
        }
        PreferencesAction::EncryptionKeyGet => {
            let result = pagbank_sdk::endpoints::preferences::get_encryption_key(&client).await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Chave de Criptografia", &result)
                }
            }
            Ok(())
        }
        PreferencesAction::EncryptionKeyCreate { body } => {
            let body_val: serde_json::Value = serde_json::from_str(&body)?;
            let result =
                pagbank_sdk::endpoints::preferences::create_encryption_key(&client, &body_val)
                    .await?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&result),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Chave de Criptografia Criada", &result)
                }
            }
            Ok(())
        }
    }
}
