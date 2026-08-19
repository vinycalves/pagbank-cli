use anyhow::Result;

use crate::cli::RetriesAction;
use crate::output;

pub async fn run(
    action: RetriesAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

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
