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
            _ => Err(format!("ambiente inválido: {s}. Use 'sandbox' ou 'production'")),
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
            (Service::Secure, Environment::Sandbox) => {
                "https://secure.sandbox.api.pagseguro.com"
            }
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
