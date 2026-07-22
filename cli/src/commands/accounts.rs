use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};

use crate::cli::AccountsAction;
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

pub async fn run(action: AccountsAction, env_override: Option<&str>, output_fmt: &crate::cli::OutputFormat) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        AccountsAction::Create { reference_id, name, email, tax_id } => {
            let body = serde_json::json!({
                "reference_id": reference_id,
                "name": name,
                "email": email,
                "tax_id": tax_id,
            });
            let result = pagbank_sdk::endpoints::accounts::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Conta Criada", &val),
            }
            Ok(())
        }
        AccountsAction::Get { id } => {
            let result = pagbank_sdk::endpoints::accounts::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Conta", &val),
            }
            Ok(())
        }
    }
}
