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
