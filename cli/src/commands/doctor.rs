use anyhow::Result;
use pagbank_sdk::PagBankError;

use crate::config::PbConfig;
use crate::output;

pub async fn run(connect: bool) -> Result<()> {
    let mut ok = true;

    let path = PbConfig::config_path()?;
    if path.exists() {
        println!("✓ Arquivo de configuração: {}", path.display());
    } else {
        println!(
            "✗ Arquivo de configuração não encontrado: {}",
            path.display()
        );
        println!("  Rode 'pb config init' para criar a configuração.");
        ok = false;
    }

    let config = match PbConfig::load() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Falha ao ler a configuração: {e}");
            print_verdict(ok);
            return Ok(());
        }
    };

    match config
        .default
        .environment
        .parse::<pagbank_sdk::Environment>()
    {
        Ok(env) => println!("✓ Ambiente: {env}"),
        Err(_) => {
            println!("✗ Ambiente inválido: '{}'", config.default.environment);
            ok = false;
        }
    }

    if config.default.token.is_empty() {
        println!("✗ Token de autenticação não configurado");
        println!("  Rode 'pb auth login --token <TOKEN>'");
        ok = false;
    } else {
        println!("✓ Token de autenticação configurado");
    }

    if config.default.recurring_token.is_none() {
        println!("⚠ Token de recorrência não configurado (necessário apenas para recorrência)");
    } else {
        println!("✓ Token de recorrência configurado");
    }

    if connect {
        match check_connectivity(&config).await {
            Ok(status) => println!("✓ API acessível (HTTP {status})"),
            Err(msg) => {
                println!("✗ {msg}");
                ok = false;
            }
        }
    }

    print_verdict(ok);
    if !ok {
        std::process::exit(1);
    }
    Ok(())
}

async fn check_connectivity(config: &PbConfig) -> Result<u16, String> {
    let client = crate::client::make_client(config, None).map_err(|e| e.to_string())?;
    match client.get(pagbank_sdk::Service::Main, "/public-keys").await {
        Ok(resp) => Ok(resp.status().as_u16()),
        Err(e) => Err(map_connectivity_error(&e)),
    }
}

fn map_connectivity_error(e: &PagBankError) -> String {
    match e {
        PagBankError::Api { status, .. } if *status == 401 => {
            "API acessível, mas o token foi rejeitado (401)".to_string()
        }
        PagBankError::Api { status, .. } => format!("API respondeu com status {status}"),
        PagBankError::NoToken => "token não configurado para testar a API".to_string(),
        other => format!("sem conexão com a API: {other}"),
    }
}

fn print_verdict(ok: bool) {
    if ok {
        output::print_success("pb doctor: nenhum problema encontrado");
    } else {
        eprintln!("\nProblemas encontrados. Corrija e rode 'pb doctor' novamente.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_connectivity_401() {
        let err = PagBankError::Api {
            status: 401,
            code: "UNAUTHORIZED".to_string(),
            message: "invalid".to_string(),
        };
        assert!(map_connectivity_error(&err).contains("401"));
        assert!(map_connectivity_error(&err).contains("rejeitado"));
    }

    #[test]
    fn map_connectivity_500() {
        let err = PagBankError::Api {
            status: 500,
            code: "INTERNAL".to_string(),
            message: "boom".to_string(),
        };
        assert!(map_connectivity_error(&err).contains("500"));
    }

    #[test]
    fn map_connectivity_no_token() {
        assert!(map_connectivity_error(&PagBankError::NoToken).contains("token"));
    }
}
