use thiserror::Error;

#[derive(Error, Debug)]
pub enum PagBankError {
    #[error("erro de rede: {0}")]
    Network(#[from] reqwest::Error),

    #[error("erro de serialização: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("erro de URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("erro de API [{status}] código {code}: {message}")]
    Api {
        status: u16,
        code: String,
        message: String,
    },

    #[error("erro de API [{status}]: {body}")]
    ApiRaw { status: u16, body: String },

    #[error("token de autenticação não configurado")]
    NoToken,

    #[error("token de recorrência não configurado")]
    NoRecurringToken,

    #[error("idempotency key inválida: {0}")]
    InvalidIdempotencyKey(String),

    #[error("erro de autenticação: {0}")]
    Auth(String),

    #[error("{0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_api() {
        let err = PagBankError::Api {
            status: 401,
            code: "UNAUTHORIZED".to_string(),
            message: "token inválido".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "erro de API [401] código UNAUTHORIZED: token inválido"
        );
    }

    #[test]
    fn error_display_api_raw() {
        let err = PagBankError::ApiRaw {
            status: 500,
            body: "internal error".to_string(),
        };
        assert_eq!(err.to_string(), "erro de API [500]: internal error");
    }

    #[test]
    fn error_display_no_token() {
        let err = PagBankError::NoToken;
        assert_eq!(err.to_string(), "token de autenticação não configurado");
    }

    #[test]
    fn error_display_no_recurring_token() {
        let err = PagBankError::NoRecurringToken;
        assert_eq!(err.to_string(), "token de recorrência não configurado");
    }

    #[test]
    fn error_display_invalid_idempotency_key() {
        let err = PagBankError::InvalidIdempotencyKey("abc".to_string());
        assert_eq!(err.to_string(), "idempotency key inválida: abc");
    }

    #[test]
    fn error_display_auth() {
        let err = PagBankError::Auth("cesso negado".to_string());
        assert_eq!(err.to_string(), "erro de autenticação: cesso negado");
    }

    #[test]
    fn error_display_other() {
        let err = PagBankError::Other("algo deu errado".to_string());
        assert_eq!(err.to_string(), "algo deu errado");
    }

    #[test]
    fn error_from_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err: PagBankError = json_err.into();
        assert!(matches!(err, PagBankError::Serialization(_)));
    }

    #[test]
    fn error_from_url_error() {
        let url_err = "not a url".parse::<url::Url>().unwrap_err();
        let err: PagBankError = url_err.into();
        assert!(matches!(err, PagBankError::Url(_)));
    }
}
