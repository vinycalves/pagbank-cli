use anyhow::Result;

use crate::cli::SubscribersAction;
use crate::output;

pub async fn run(
    action: SubscribersAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let client = crate::client::load_client(env_override)?;

    match action {
        SubscribersAction::Create {
            name,
            email,
            tax_id,
            reference_id,
            phone_area,
            phone_number,
            phone_type,
        } => {
            if let Err(msg) = crate::validators::validate_email(&email) {
                anyhow::bail!(msg);
            }
            if let Err(msg) = crate::validators::validate_tax_id(&tax_id) {
                anyhow::bail!(msg);
            }
            let mut body = serde_json::json!({
                "name": name,
                "email": email,
                "tax_id": tax_id,
            });
            if let Some(ri) = reference_id {
                body["reference_id"] = serde_json::json!(ri);
            }
            if let (Some(area), Some(number)) = (phone_area, phone_number) {
                let pt = phone_type.unwrap_or_else(|| "MOBILE".to_string());
                body["phones"] = serde_json::json!([{
                    "country": "55",
                    "area": area,
                    "number": number,
                    "type": pt,
                }]);
            }
            let result = pagbank_sdk::endpoints::subscribers::create(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Assinante Criado", &val)
                }
            }
            Ok(())
        }
        SubscribersAction::Get { id } => {
            let result = pagbank_sdk::endpoints::subscribers::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Assinante", &val),
            }
            Ok(())
        }
        SubscribersAction::List { page, per_page } => {
            let params = vec![
                ("page".to_string(), page.to_string()),
                ("per_page".to_string(), per_page.to_string()),
            ];
            let result = pagbank_sdk::endpoints::subscribers::list(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr
                            .iter()
                            .map(|s| {
                                vec![
                                    s["id"].as_str().unwrap_or("").to_string(),
                                    s["name"].as_str().unwrap_or("").to_string(),
                                    s["email"].as_str().unwrap_or("").to_string(),
                                ]
                            })
                            .collect();
                        output::print_table(&["ID", "Nome", "Email"], rows);
                    }
                }
            }
            Ok(())
        }
        SubscribersAction::UpdateProfile {
            id,
            name,
            email,
            tax_id,
        } => {
            let mut body = serde_json::json!({});
            if let Some(n) = name {
                body["name"] = serde_json::json!(n);
            }
            if let Some(e) = email {
                body["email"] = serde_json::json!(e);
            }
            if let Some(t) = tax_id {
                body["tax_id"] = serde_json::json!(t);
            }
            let result = pagbank_sdk::endpoints::subscribers::update(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Assinante Atualizado", &val)
                }
            }
            Ok(())
        }
        SubscribersAction::UpdatePayment {
            id,
            card_number,
            card_exp_month,
            card_exp_year,
            card_cvv,
            card_holder_name,
            card_holder_tax_id,
        } => {
            let body = serde_json::json!({
                "payment_method": {
                    "credit_card": {
                        "number": card_number,
                        "exp_month": card_exp_month,
                        "exp_year": card_exp_year,
                        "security_code": card_cvv,
                        "holder": {
                            "name": card_holder_name,
                            "tax_id": card_holder_tax_id,
                        }
                    }
                }
            });
            let result =
                pagbank_sdk::endpoints::subscribers::update_payment(&client, &id, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Pagamento Atualizado", &val)
                }
            }
            Ok(())
        }
    }
}
