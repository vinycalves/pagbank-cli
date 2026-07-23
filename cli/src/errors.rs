use pagbank_sdk::PagBankError;

pub fn translate(err: &PagBankError) -> String {
    match err {
        PagBankError::Api { status, code, message } => {
            let friendly = api_message(status, code, message);
            format!("[{}] {friendly}", status)
        }
        PagBankError::ApiRaw { status, body } => {
            let friendly = api_raw_message(status, body);
            format!("[{}] {friendly}", status)
        }
        PagBankError::Network(_) => "erro de rede: verifique sua conexão com a internet".to_string(),
        PagBankError::Serialization(_) => "erro interno ao processar a resposta da API".to_string(),
        PagBankError::Url(_) => "URL inválida na requisição".to_string(),
        PagBankError::NoToken => "token de autenticação não configurado. Use 'pb auth login --token <TOKEN>'".to_string(),
        PagBankError::NoRecurringToken => "token de recorrência não configurado. Configure 'recurring_token' no config.toml".to_string(),
        PagBankError::InvalidIdempotencyKey(_) => format!("{}", err),
        PagBankError::Auth(msg) => format!("erro de autenticação: {msg}"),
        PagBankError::Other(msg) => msg.clone(),
    }
}

fn api_message(status: &u16, _code: &str, message: &str) -> String {
    let friendly = translate_description(message);
    if friendly != message {
        return friendly;
    }

    if let Some((param, detail)) = api_detail(message) {
        return match param {
            Some(p) => format!("parâmetro '{p}': {detail}"),
            None => detail,
        };
    }

    match *status {
        401 => "token inválido. Use 'pb auth login --token <TOKEN>' com um token válido".to_string(),
        403 => "sem permissão para acessar este recurso na sua conta PagBank".to_string(),
        404 => "recurso não encontrado. Verifique o ID informado".to_string(),
        422 => "dados inválidos. Verifique os campos enviados".to_string(),
        429 => "muitas requisições. Aguarde alguns segundos e tente novamente".to_string(),
        500..=599 => "erro interno do PagBank. Tente novamente mais tarde".to_string(),
        _ => message.to_string(),
    }
}

fn api_raw_message(status: &u16, body: &str) -> String {
    match status {
        401 => "token inválido ou não autorizado".to_string(),
        403 => "sem permissão para acessar este recurso".to_string(),
        404 => "recurso não encontrado".to_string(),
        429 => "muitas requisições. Aguarde alguns segundos".to_string(),
        500..=599 => "erro interno do PagBank. Tente novamente".to_string(),
        _ => format!("erro inesperado: {body}"),
    }
}

fn api_detail(message: &str) -> Option<(Option<String>, String)> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(message) {
        if let Some(errors) = json.get("error_messages").and_then(|v| v.as_array()) {
            for e in errors {
                let desc = e.get("description").and_then(|v| v.as_str());
                let param = e.get("parameter_name").and_then(|v| v.as_str());
                if let Some(d) = desc {
                    return Some((param.map(String::from), translate_description(d)));
                }
            }
        }
    }
    None
}

fn split_flat(desc: &str) -> (&str, &str) {
    if desc.starts_with('{') {
        return ("", desc);
    }
    let parts: Vec<&str> = desc.splitn(2, ": ").collect();
    if parts.len() == 2 {
        (parts[0].trim(), parts[1].trim())
    } else {
        ("", desc)
    }
}

fn translate_description(desc: &str) -> String {
    let (param, description) = split_flat(desc);

    if description.contains("must have at least 1 element") {
        return if param.is_empty() {
            "é necessário informar ao menos 1 item".to_string()
        } else {
            format!("é necessário informar ao menos 1 item em '{param}'")
        };
    }

    if description.contains("Invalid credential")
        || description.contains("Review AUTHORIZATION")
    {
        return "token inválido. Use 'pb auth login --token <TOKEN>' com um token válido".to_string();
    }

    if description.contains("No known parameter was given")
        || description.contains("Invalid search parameters")
    {
        return if param.is_empty() {
            "nenhum parâmetro de busca informado".to_string()
        } else {
            format!("parâmetro '{param}': nenhum parâmetro de busca informado")
        };
    }

    if description.contains("not authorized")
        || description.contains("explicit deny")
    {
        return "token sem permissão para este recurso na sua conta PagBank".to_string();
    }

    if description.contains("not found") {
        return "recurso não encontrado. Verifique o ID informado".to_string();
    }

    if description.contains("rate limit")
        || description.contains("too many requests")
    {
        return "muitas requisições. Aguarde alguns segundos e tente novamente".to_string();
    }

    desc.to_string()
}

pub fn validate_order_create(
    method: &str,
    _qr_amount: Option<i64>,
) -> Result<(), String> {
    match method.to_lowercase().as_str() {
        "pix" => Ok(()),
        "credit_card" | "debit_card" => Ok(()),
        _ => Err(format!("método de pagamento inválido: {method}. Use pix, credit_card ou debit_card")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_400_qr_codes() {
        let err = PagBankError::Api {
            status: 400,
            code: "40002".to_string(),
            message: "qr_codes: must have at least 1 element".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("qr_codes"));
    }

    #[test]
    #[test]
    fn translate_400_qr_codes_raw_json() {
        let err = PagBankError::Api {
            status: 400,
            code: "40002".to_string(),
            message: "qr_codes: must have at least 1 element".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("qr_codes"));
    }

    #[test]
    fn translate_403_message() {
        let err = PagBankError::Api {
            status: 403,
            code: "ACCESS_DENIED".to_string(),
            message: "User is not authorized to access this resource with an explicit deny in an identity-based policy".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("sem permissão"));
    }

    #[test]
    fn translate_401() {
        let err = PagBankError::Api {
            status: 401,
            code: "UNAUTHORIZED".to_string(),
            message: "invalid token".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("token inválido"));
    }

    #[test]
    fn translate_403() {
        let err = PagBankError::Api {
            status: 403,
            code: "FORBIDDEN".to_string(),
            message: "denied".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("sem permissão"));
    }

    #[test]
    fn translate_404() {
        let err = PagBankError::Api {
            status: 404,
            code: "NOT_FOUND".to_string(),
            message: "order not found".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("não encontrado"));
    }

    #[test]
    fn translate_429() {
        let err = PagBankError::Api {
            status: 429,
            code: "RATE_LIMIT".to_string(),
            message: "too many".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("muitas requisições"));
    }

    #[test]
    fn translate_500() {
        let err = PagBankError::Api {
            status: 500,
            code: "INTERNAL".to_string(),
            message: "server error".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("erro interno do PagBank"));
    }

    #[test]
    fn translate_no_token() {
        let err = PagBankError::NoToken;
        let msg = translate(&err);
        assert!(msg.contains("auth login"));
    }

    #[test]
    fn translate_api_raw_401() {
        let err = PagBankError::ApiRaw {
            status: 401,
            body: "unauthorized".to_string(),
        };
        let msg = translate(&err);
        assert!(msg.contains("token inválido"));
    }



    #[test]
    fn validate_order_create_pix_without_qr() {
        let result = validate_order_create("pix", None);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_order_create_pix_with_qr() {
        let result = validate_order_create("pix", Some(100));
        assert!(result.is_ok());
    }

    #[test]
    fn validate_order_create_credit_card() {
        let result = validate_order_create("credit_card", None);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_order_create_invalid_method() {
        let result = validate_order_create("boleto", None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("método"));
    }
}
