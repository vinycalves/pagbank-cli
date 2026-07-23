use anyhow::Result;

use crate::cli::AuthAction;
use crate::config::PbConfig;
use crate::output;

pub async fn run(action: AuthAction) -> Result<()> {
    match action {
        AuthAction::Login { token } => {
            let mut config = PbConfig::load()?;
            config.default.token = token;
            config.save()?;
            output::print_success("Token configurado com sucesso");
            Ok(())
        }
        AuthAction::Logout => {
            let mut config = PbConfig::load()?;
            config.default.token.clear();
            config.save()?;
            output::print_success("Credenciais removidas");
            Ok(())
        }
        AuthAction::Status => {
            let config = PbConfig::load()?;
            let env = &config.default.environment;
            let token = &config.default.token;
            if token.is_empty() {
                output::print_error("Nenhum token configurado");
            } else {
                let masked = format!(
                    "{}...{}",
                    &token[..8.min(token.len())],
                    &token[token.len().saturating_sub(4)..]
                );
                output::print_info(&format!("Ambiente: {env}"));
                output::print_info(&format!("Token: {masked}"));
                if config.default.recurring_token.is_some() {
                    output::print_info("Token de recorrência: configurado");
                }
                if config.default.client_id.is_some() {
                    output::print_info("Client ID (Connect): configurado");
                }
                output::print_info(
                    "Para testar a conexão com a API, utilize um comando como 'pb orders list'",
                );
            }
            Ok(())
        }
    }
}
