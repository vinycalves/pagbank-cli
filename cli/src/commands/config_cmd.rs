use anyhow::Result;

use crate::cli::ConfigAction;
use crate::config::PbConfig;
use crate::output;

pub async fn run(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Init => {
            let token = dialoguer::Input::<String>::new()
                .with_prompt("Token de autenticação (sandbox)")
                .interact()?;
            let environment = dialoguer::Select::new()
                .with_prompt("Ambiente padrão")
                .items(&["sandbox", "production"])
                .default(0)
                .interact()?;

            let env_str = if environment == 0 {
                "sandbox"
            } else {
                "production"
            };

            let recurring_token = dialoguer::Input::<String>::new()
                .with_prompt("Token de recorrência (vazio para pular)")
                .allow_empty(true)
                .default(String::new())
                .interact()?;

            let mut config = PbConfig::load()?;
            config.default.token = token;
            config.default.environment = env_str.to_string();
            if !recurring_token.is_empty() {
                config.default.recurring_token = Some(recurring_token);
            }
            config.save()?;
            output::print_success("Configuração inicial concluída!");
            Ok(())
        }
        ConfigAction::Set { key, value } => {
            let mut config = PbConfig::load()?;
            config.set_value(&key, &value)?;
            output::print_success(&format!("{key} definido com sucesso"));
            Ok(())
        }
        ConfigAction::Get { key } => {
            let config = PbConfig::load()?;
            let value = config.get_value(&key)?;
            println!("{value}");
            Ok(())
        }
        ConfigAction::Show => {
            let config = PbConfig::load()?;
            println!("Ambiente: {}", config.default.environment);
            let token_display = if config.default.token.is_empty() {
                "(não configurado)"
            } else {
                "***"
            };
            println!("Token: {token_display}");
            let recurring_display = if config.default.recurring_token.is_some() {
                "configurado"
            } else {
                "(não configurado)"
            };
            println!("Token recorrência: {recurring_display}");
            println!(
                "Client ID: {}",
                config
                    .default
                    .client_id
                    .as_deref()
                    .unwrap_or("(não configurado)")
            );
            Ok(())
        }
    }
}
