use anyhow::Result;

use crate::cli::PreferencesAction;
use crate::output;

pub async fn run(
    action: PreferencesAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

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
