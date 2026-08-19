use anyhow::Result;

use crate::cli::CertsAction;
use crate::output;

pub async fn run(
    action: CertsAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        CertsAction::Create {
            certificate,
            password,
        } => {
            let body = serde_json::json!({
                "certificate": certificate,
                "password": password,
            });
            let result = pagbank_sdk::endpoints::certificates::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Certificado Criado", &val)
                }
            }
            Ok(())
        }
    }
}
