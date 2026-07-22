use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::cli::ConnectAction;
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
    action: ConnectAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        ConnectAction::AppCreate {
            name,
            description,
            site,
            redirect_uri,
            logo,
        } => {
            let mut body = serde_json::json!({ "name": name });
            if let Some(d) = description {
                body["description"] = serde_json::json!(d);
            }
            if let Some(s) = site {
                body["site"] = serde_json::json!(s);
            }
            if let Some(r) = redirect_uri {
                body["redirect_uri"] = serde_json::json!(r);
            }
            if let Some(l) = logo {
                body["logo"] = serde_json::json!(l);
            }
            let result = pagbank_sdk::endpoints::connect::create_app(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Aplicação Connect Criada", &val)
                }
            }
            Ok(())
        }
        ConnectAction::AppGet { id } => {
            let result = pagbank_sdk::endpoints::connect::get_app(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Aplicação Connect", &val)
                }
            }
            Ok(())
        }
        ConnectAction::Authorize {
            app_id,
            redirect_uri,
            scope,
        } => {
            let url = pagbank_sdk::endpoints::connect::get_authorize_url(
                &client,
                &app_id,
                &redirect_uri,
                &scope,
            )
            .await;
            output::print_info(&format!("URL de autorização:\n{url}"));
            Ok(())
        }
        ConnectAction::Token { code } => {
            let client_id = config.default.client_id.as_deref().unwrap_or("");
            let client_secret = config.default.client_secret.as_deref().unwrap_or("");
            let body = serde_json::json!({
                "grant_type": "authorization_code",
                "code": code,
                "client_id": client_id,
                "client_secret": client_secret,
            });
            let result = pagbank_sdk::endpoints::connect::create_token(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Access Token", &val),
            }
            Ok(())
        }
        ConnectAction::TokenRefresh { refresh_token } => {
            let client_id = config.default.client_id.as_deref().unwrap_or("");
            let client_secret = config.default.client_secret.as_deref().unwrap_or("");
            let body = serde_json::json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,
                "client_id": client_id,
                "client_secret": client_secret,
            });
            let result = pagbank_sdk::endpoints::connect::refresh_token(&client, &body).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    output::print_object_table("Token Renovado", &val)
                }
            }
            Ok(())
        }
        ConnectAction::TokenRevoke { token } => {
            let body = serde_json::json!({ "token": token });
            pagbank_sdk::endpoints::connect::revoke_token(&client, &body).await?;
            output::print_success("Token revogado com sucesso");
            Ok(())
        }
    }
}
