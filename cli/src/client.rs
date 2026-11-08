use anyhow::Result;
use pagbank_sdk::{Environment, PagBankClient, PagBankConfig};

use crate::config::PbConfig;

pub fn make_client(config: &PbConfig, env_override: Option<&str>) -> Result<PagBankClient> {
    let active = config.get_active_config(env_override);
    let pagbank_config = PagBankConfig {
        environment: active.environment.parse().unwrap_or(Environment::Sandbox),
        token: active.token.clone(),
        recurring_token: active.recurring_token.clone(),
        transfer_token: active.transfer_token.clone(),
        edi_user: active.edi_user.clone(),
        edi_token: active.edi_token.clone(),
        client_id: active.client_id.clone(),
        client_secret: active.client_secret.clone(),
    };
    Ok(PagBankClient::new(pagbank_config))
}

pub fn load_client(env_override: Option<&str>) -> Result<PagBankClient> {
    let config = PbConfig::load()?;
    make_client(&config, env_override)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pagbank_sdk::Environment;

    #[test]
    fn make_client_uses_active_env() {
        let mut config = PbConfig::default();
        config.default.token = "abc".to_string();
        config.default.environment = "sandbox".to_string();
        let client = make_client(&config, None).unwrap();
        assert_eq!(client.config.environment, Environment::Sandbox);
        assert_eq!(client.config.token, "abc");
    }

    #[test]
    fn make_client_production_override() {
        let config = PbConfig::default();
        let client = make_client(&config, Some("production")).unwrap();
        assert_eq!(client.config.environment, Environment::Sandbox);
    }

    #[test]
    fn make_client_maps_transfer_and_edi_credentials() {
        let mut config = PbConfig::default();
        config.default.transfer_token = Some("transfer".to_string());
        config.default.edi_user = Some("123".to_string());
        config.default.edi_token = Some("edi".to_string());
        let client = make_client(&config, None).unwrap();
        assert_eq!(client.config.transfer_token.as_deref(), Some("transfer"));
        assert_eq!(client.config.edi_user.as_deref(), Some("123"));
        assert_eq!(client.config.edi_token.as_deref(), Some("edi"));
    }
}
