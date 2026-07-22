use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Sandbox => write!(f, "sandbox"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl std::str::FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sandbox" => Ok(Environment::Sandbox),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "ambiente inválido: {s}. Use 'sandbox' ou 'production'"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    Main,
    Recurring,
    Secure,
}

impl Service {
    pub fn base_url(&self, env: &Environment) -> &'static str {
        match (self, env) {
            (Service::Main, Environment::Sandbox) => "https://sandbox.api.pagseguro.com",
            (Service::Main, Environment::Production) => "https://api.pagseguro.com",
            (Service::Recurring, Environment::Sandbox) => {
                "https://sandbox.api.assinaturas.pagseguro.com"
            }
            (Service::Recurring, Environment::Production) => {
                "https://api.assinaturas.pagseguro.com"
            }
            (Service::Secure, Environment::Sandbox) => "https://secure.sandbox.api.pagseguro.com",
            (Service::Secure, Environment::Production) => "https://secure.api.pagseguro.com",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PagBankConfig {
    pub environment: Environment,
    pub token: String,
    pub recurring_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl Default for PagBankConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Sandbox,
            token: String::new(),
            recurring_token: None,
            client_id: None,
            client_secret: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_from_str_sandbox() {
        assert_eq!("sandbox".parse::<Environment>().unwrap(), Environment::Sandbox);
    }

    #[test]
    fn environment_from_str_production() {
        assert_eq!("production".parse::<Environment>().unwrap(), Environment::Production);
    }

    #[test]
    fn environment_from_str_case_insensitive() {
        assert_eq!("Sandbox".parse::<Environment>().unwrap(), Environment::Sandbox);
        assert_eq!("PRODUCTION".parse::<Environment>().unwrap(), Environment::Production);
    }

    #[test]
    fn environment_from_str_invalid() {
        assert!("invalid".parse::<Environment>().is_err());
        assert!("".parse::<Environment>().is_err());
    }

    #[test]
    fn environment_display() {
        assert_eq!(Environment::Sandbox.to_string(), "sandbox");
        assert_eq!(Environment::Production.to_string(), "production");
    }

    #[test]
    fn service_base_url_sandbox() {
        assert_eq!(
            Service::Main.base_url(&Environment::Sandbox),
            "https://sandbox.api.pagseguro.com"
        );
        assert_eq!(
            Service::Recurring.base_url(&Environment::Sandbox),
            "https://sandbox.api.assinaturas.pagseguro.com"
        );
        assert_eq!(
            Service::Secure.base_url(&Environment::Sandbox),
            "https://secure.sandbox.api.pagseguro.com"
        );
    }

    #[test]
    fn service_base_url_production() {
        assert_eq!(
            Service::Main.base_url(&Environment::Production),
            "https://api.pagseguro.com"
        );
        assert_eq!(
            Service::Recurring.base_url(&Environment::Production),
            "https://api.assinaturas.pagseguro.com"
        );
        assert_eq!(
            Service::Secure.base_url(&Environment::Production),
            "https://secure.api.pagseguro.com"
        );
    }

    #[test]
    fn pagbank_config_default() {
        let config = PagBankConfig::default();
        assert_eq!(config.environment, Environment::Sandbox);
        assert_eq!(config.token, "");
        assert!(config.recurring_token.is_none());
        assert!(config.client_id.is_none());
        assert!(config.client_secret.is_none());
    }
}
