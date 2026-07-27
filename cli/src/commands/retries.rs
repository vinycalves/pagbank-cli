use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::RetriesAction;
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
    action: RetriesAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        RetriesAction::Get { id } => {
            let result = pagbank_sdk::endpoints::retries::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Retentativa", &val),
            }
            Ok(())
        }
        RetriesAction::Update { id, body } => {
            let body_val: serde_json::Value = serde_json::from_str(&body)?;
            let result = pagbank_sdk::endpoints::retries::update(&client, &id, &body_val).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Retentativa Atualizada", &val)
                }
            }
            Ok(())
        }
        RetriesAction::ManualRetry { subs_id } => {
            pagbank_sdk::endpoints::retries::manual_retry(&client, &subs_id).await?;
            output::print_success("Retentativa manual disparada");
            Ok(())
        }
    }
}
