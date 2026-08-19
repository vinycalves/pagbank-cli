use anyhow::Result;

use crate::cli::KeysAction;
use crate::output;

pub async fn run(
    action: KeysAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        KeysAction::Create { r#type } => {
            let result = pagbank_sdk::endpoints::public_keys::create(&client, &r#type).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Chave Pública Criada", &val)
                }
            }
            Ok(())
        }
        KeysAction::Get { id } => {
            let result = pagbank_sdk::endpoints::public_keys::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Chave Pública", &val)
                }
            }
            Ok(())
        }
        KeysAction::Update { id } => {
            let body = serde_json::json!({});
            let result = pagbank_sdk::endpoints::public_keys::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Chave Pública Atualizada", &val)
                }
            }
            Ok(())
        }
    }
}
