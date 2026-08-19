use anyhow::Result;

use crate::cli::AccountsAction;
use crate::output;

pub async fn run(
    action: AccountsAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        AccountsAction::Create {
            reference_id,
            name,
            email,
            tax_id,
            r#type,
            tos_ip,
        } => {
            if let Err(msg) = crate::validators::validate_email(&email) {
                anyhow::bail!(msg);
            }
            if let Err(msg) = crate::validators::validate_tax_id(&tax_id) {
                anyhow::bail!(msg);
            }
            use chrono::Utc;
            let body = serde_json::json!({
                "reference_id": reference_id,
                "name": name,
                "email": email,
                "tax_id": tax_id,
                "type": r#type,
                "tos_acceptance": {
                    "ip": tos_ip,
                    "accepted_at": Utc::now().to_rfc3339(),
                },
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
