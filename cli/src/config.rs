use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PbConfig {
    pub default: EnvironmentConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production: Option<EnvironmentConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("não foi possível determinar o diretório de configuração"))?;
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
