use anyhow::{bail, Result};

use crate::cli::TransfersAction;
use crate::output;

#[allow(clippy::too_many_arguments)]
fn build_instrument(
    transfer_type: &str,
    bank_branch: Option<&str>,
    account_number: Option<&str>,
    account_id: Option<&str>,
    email: Option<&str>,
    pix_key: Option<&str>,
    pix_name: Option<&str>,
    pix_tax_id: Option<&str>,
    bank_ispb: Option<&str>,
    account_type: Option<&str>,
) -> Result<serde_json::Value> {
    match transfer_type {
        "P2P" => {
            if account_id.is_none() && (bank_branch.is_none() || account_number.is_none()) {
                bail!(
                    "transferência P2P exige --account-id ou o par --bank-branch/--account-number"
                );
            }
            let mut p2p = serde_json::Map::new();
            if let Some(v) = bank_branch {
                p2p.insert("bank_branch".into(), v.into());
            }
            if let Some(v) = account_number {
                p2p.insert("account_number".into(), v.into());
            }
            if let Some(v) = account_id {
                p2p.insert("account_id".into(), v.into());
            }
            if let Some(v) = email {
                p2p.insert("email".into(), v.into());
            }
            Ok(serde_json::json!({"type": "P2P", "p2p": p2p}))
        }
        "PIX" => {
            if pix_key.is_none()
                && (pix_tax_id.is_none()
                    || bank_ispb.is_none()
                    || bank_branch.is_none()
                    || account_number.is_none()
                    || account_type.is_none())
            {
                bail!(
                    "transferência PIX exige --pix-key ou os dados bancários (--pix-tax-id, --bank-ispb, --bank-branch, --account-number e --account-type)"
                );
            }
            let mut pix = serde_json::Map::new();
            for (key, value) in [
                ("key", pix_key),
                ("name", pix_name),
                ("tax_id", pix_tax_id),
                ("bank_ispb", bank_ispb),
                ("bank_branch", bank_branch),
                ("account_number", account_number),
                ("account_type", account_type),
            ] {
                if let Some(v) = value {
                    pix.insert(key.into(), v.into());
                }
            }
            Ok(serde_json::json!({"type": "PIX", "pix": pix}))
        }
        other => bail!("tipo de transferência inválido: {other}. Use P2P ou PIX"),
    }
}

pub async fn run(
    action: TransfersAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        TransfersAction::Create {
            amount,
            r#type,
            description,
            reference_id,
            notification_url,
            bank_branch,
            account_number,
            account_id,
            email,
            pix_key,
            pix_name,
            pix_tax_id,
            bank_ispb,
            account_type,
        } => {
            if amount <= 0 {
                bail!("o valor da transferência deve ser positivo (em centavos)");
            }
            let instrument = build_instrument(
                &r#type,
                bank_branch.as_deref(),
                account_number.as_deref(),
                account_id.as_deref(),
                email.as_deref(),
                pix_key.as_deref(),
                pix_name.as_deref(),
                pix_tax_id.as_deref(),
                bank_ispb.as_deref(),
                account_type.as_deref(),
            )?;

            let mut body = serde_json::json!({
                "amount": {"value": amount, "currency": "BRL"},
                "instrument": instrument,
                "notification_urls": [notification_url],
            });
            if let Some(v) = description {
                body["description"] = v.into();
            }
            if let Some(v) = reference_id {
                body["reference_id"] = v.into();
            }

            let result =
                pagbank_sdk::endpoints::transfers::create(&client, &body, &Default::default())
                    .await?;
            let val = serde_json::to_value(result)?;
            output::print_success("Transferência solicitada");
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Transferência", &val)
                }
            }
            Ok(())
        }
        TransfersAction::Get { id } => {
            let result = pagbank_sdk::endpoints::transfers::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Transferência", &val)
                }
            }
            Ok(())
        }
        TransfersAction::List {
            reference_id,
            start_date,
            end_date,
            hash,
            size,
        } => {
            let mut params: Vec<(String, String)> = Vec::new();
            for (key, value) in [
                ("reference_id", reference_id),
                ("start_date", start_date),
                ("end_date", end_date),
                ("hash", hash),
            ] {
                if let Some(v) = value {
                    params.push((key.to_string(), v));
                }
            }
            if let Some(v) = size {
                params.push(("size".to_string(), v.to_string()));
            }
            let result = pagbank_sdk::endpoints::transfers::list(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|t| {
                                vec![
                                    t["id"].as_str().unwrap_or("").to_string(),
                                    t["status"].as_str().unwrap_or("").to_string(),
                                    t["amount"]["value"].to_string(),
                                    t["description"].as_str().unwrap_or("").to_string(),
                                    t["created_at"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(
                            &["ID", "Status", "Valor", "Descrição", "Criado em"],
                            rows,
                        );
                    }
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instrument_p2p_with_account_id() {
        let instrument = build_instrument(
            "P2P",
            None,
            None,
            Some("ACCO_1"),
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(instrument["type"], "P2P");
        assert_eq!(instrument["p2p"]["account_id"], "ACCO_1");
    }

    #[test]
    fn instrument_p2p_requires_destination() {
        let result = build_instrument("P2P", None, None, None, None, None, None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn instrument_pix_with_key() {
        let instrument = build_instrument(
            "PIX",
            None,
            None,
            None,
            None,
            Some("chave@pix.com"),
            Some("Maria"),
            Some("12345678909"),
            None,
            None,
        )
        .unwrap();
        assert_eq!(instrument["type"], "PIX");
        assert_eq!(instrument["pix"]["key"], "chave@pix.com");
        assert_eq!(instrument["pix"]["name"], "Maria");
    }

    #[test]
    fn instrument_pix_requires_key_or_bank_data() {
        let result = build_instrument("PIX", None, None, None, None, None, None, None, None, None);
        assert!(result.is_err());
    }
}
