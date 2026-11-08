use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::cli::ChargebacksAction;
use crate::output;

const MAX_DISPUTE_BYTES: usize = 4 * 1024 * 1024;

fn validate_dispute_file(path: &str, len: usize) -> Result<()> {
    if !path.to_lowercase().ends_with(".pdf") {
        bail!("a evidência da disputa deve ser um arquivo PDF");
    }
    if len > MAX_DISPUTE_BYTES {
        bail!("o PDF excede o limite de 4 MB");
    }
    Ok(())
}

pub async fn run(
    action: ChargebacksAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        ChargebacksAction::Get { id } => {
            let result = pagbank_sdk::endpoints::chargebacks::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Chargeback", &val),
            }
            Ok(())
        }
        ChargebacksAction::Dispute { id, file } => {
            let bytes = std::fs::read(&file)
                .with_context(|| format!("falha ao ler o arquivo de evidência: {file}"))?;
            validate_dispute_file(&file, bytes.len())?;
            let file_name = Path::new(&file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("evidencia.pdf")
                .to_string();

            let result = pagbank_sdk::endpoints::chargebacks::create_dispute(
                &client, &id, &file_name, bytes,
            )
            .await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Disputa enviada");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Disputa", &val),
            }
            Ok(())
        }
        ChargebacksAction::GetDispute {
            chargeback_id,
            dispute_id,
        } => {
            let result = pagbank_sdk::endpoints::chargebacks::get_dispute(
                &client,
                &chargeback_id,
                &dispute_id,
            )
            .await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Disputa", &val),
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_dispute_file_accepts_pdf() {
        assert!(validate_dispute_file("defesa.PDF", 1024).is_ok());
    }

    #[test]
    fn validate_dispute_file_rejects_non_pdf() {
        let err = validate_dispute_file("defesa.jpg", 1024).unwrap_err();
        assert!(err.to_string().contains("PDF"));
    }

    #[test]
    fn validate_dispute_file_rejects_large_file() {
        let err = validate_dispute_file("defesa.pdf", MAX_DISPUTE_BYTES + 1).unwrap_err();
        assert!(err.to_string().contains("4 MB"));
    }
}
