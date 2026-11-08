use crate::client::PagBankClient;
use crate::error::PagBankError;
use crate::models::edi::EdiMovements;

/// Consulta o extrato EDI de um único dia.
///
/// `movement_type` aceita `transactional`, `financial`, `cashouts` ou `balances`.
/// A data deve estar no formato `AAAA-MM-DD`. O EDI não possui sandbox.
pub async fn movements(
    client: &PagBankClient,
    movement_type: &str,
    date: &str,
    page: i32,
    page_size: i32,
) -> Result<EdiMovements, PagBankError> {
    let path = movements_path(movement_type, date, page, page_size);
    let resp = client.get_edi(&path).await?;
    let validado_header = resp
        .headers()
        .get("validado")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("true"));
    let json: serde_json::Value = resp.json().await?;
    let mut result: EdiMovements = serde_json::from_value(json)?;
    if result.validado.is_none() {
        // A API expõe a integralidade do dia também no header `VALIDADO`.
        result.validado = validado_header;
    }
    Ok(result)
}

pub fn movements_path(movement_type: &str, date: &str, page: i32, page_size: i32) -> String {
    format!(
        "/movement/v3.00/{}/{date}?pageNumber={page}&pageSize={page_size}",
        urlencoding::encode(movement_type)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movements_path_builds_url() {
        assert_eq!(
            movements_path("transactional", "2026-11-06", 1, 1000),
            "/movement/v3.00/transactional/2026-11-06?pageNumber=1&pageSize=1000"
        );
    }

    #[test]
    fn movements_path_encodes_type() {
        assert_eq!(
            movements_path("cash outs", "2026-11-06", 2, 50),
            "/movement/v3.00/cash%20outs/2026-11-06?pageNumber=2&pageSize=50"
        );
    }
}
