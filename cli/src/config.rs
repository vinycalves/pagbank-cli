use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            environment: "sandbox".to_string(),
            token: String::new(),
            recurring_token: None,
            client_id: None,
            client_secret: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PbConfig {
    pub default: EnvironmentConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<EnvironmentConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentConfig {
    #[serde(default = "default_environment")]
    pub environment: String,
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

fn default_environment() -> String {
    "sandbox".to_string()
}

impl PbConfig {
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            anyhow::anyhow!("não foi possível determinar o diretório de configuração")
        })?;
        Ok(config_dir.join("pb"))
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        let config: PbConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        fs::create_dir_all(&dir)?;
        let path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_active_config(&self, env_override: Option<&str>) -> &EnvironmentConfig {
        match env_override {
            Some("production") => self.production.as_ref().unwrap_or(&self.default),
            _ => &self.default,
        }
    }

    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "token" => self.default.token = value.to_string(),
            "environment" => self.default.environment = value.to_string(),
            "recurring_token" => self.default.recurring_token = Some(value.to_string()),
            "client_id" => self.default.client_id = Some(value.to_string()),
            "client_secret" => self.default.client_secret = Some(value.to_string()),
            _ => anyhow::bail!("chave desconhecida: {key}"),
        }
        self.save()
    }

    pub fn get_value(&self, key: &str) -> Result<String> {
        let cfg = &self.default;
        match key {
            "token" => Ok(cfg.token.clone()),
            "environment" => Ok(cfg.environment.clone()),
            "recurring_token" => Ok(cfg.recurring_token.clone().unwrap_or_default()),
            "client_id" => Ok(cfg.client_id.clone().unwrap_or_default()),
            "client_secret" => Ok(cfg.client_secret.clone().unwrap_or_default()),
            _ => anyhow::bail!("chave desconhecida: {key}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default() {
        let config = PbConfig::default();
        assert_eq!(config.default.environment, "sandbox");
        assert_eq!(config.default.token, "");
        assert!(config.production.is_none());
    }

    #[test]
    fn config_toml_roundtrip() {
        let config = PbConfig {
            default: EnvironmentConfig {
                environment: "sandbox".to_string(),
                token: "abc123".to_string(),
                recurring_token: Some("def456".to_string()),
                client_id: None,
                client_secret: None,
            },
            production: Some(EnvironmentConfig {
                environment: "production".to_string(),
                token: "prod_token".to_string(),
                recurring_token: None,
                client_id: Some("app_id".to_string()),
                client_secret: Some("secret".to_string()),
            }),
        };

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: PbConfig = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.default.environment, "sandbox");
        assert_eq!(parsed.default.token, "abc123");
        assert_eq!(parsed.default.recurring_token, Some("def456".to_string()));
        assert!(parsed.default.client_id.is_none());
        let prod = parsed.production.unwrap();
        assert_eq!(prod.client_id, Some("app_id".to_string()));
    }

    #[test]
    fn config_minimal_toml() {
        let toml_str = r#"
            [default]
            token = "my_token"
        "#;
        let config: PbConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.default.token, "my_token");
        assert_eq!(config.default.environment, "sandbox");
    }

    #[test]
    fn test_get_active_config_default() {
        let config = PbConfig::default();
        let active = config.get_active_config(None);
        assert_eq!(active.environment, "sandbox");
    }

    #[test]
    fn test_get_active_config_production_without_override() {
        let config = PbConfig::default();
        let active = config.get_active_config(Some("sandbox"));
        assert_eq!(active.environment, "sandbox");
    }

    #[test]
    fn test_get_active_config_production_fallsback_to_default() {
        let config = PbConfig::default();
        let active = config.get_active_config(Some("production"));
        // falls back to default when production is None
        assert_eq!(active.environment, "sandbox");
    }

    #[test]
    fn test_get_value_returns_empty_for_unset() {
        let config = PbConfig::default();
        assert_eq!(config.get_value("token").unwrap(), "");
        assert_eq!(config.get_value("recurring_token").unwrap(), "");
    }

    #[test]
    fn test_get_value_unknown_key() {
        let config = PbConfig::default();
        assert!(config.get_value("unknown").is_err());
    }
}
