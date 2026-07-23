mod cli;
mod commands;
mod config;
mod errors;
mod output;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    if cli.verbose {
        eprintln!("pb: modo verbose habilitado");
    }

    let env_override = cli.environment.as_deref();
    let output_fmt = &cli.output;

    let result = match cli.command {
        cli::Commands::Auth { action } => commands::auth::run(action).await,
        cli::Commands::Config { action } => commands::config_cmd::run(action).await,
        cli::Commands::Keys { action } => {
            commands::keys::run(action, env_override, output_fmt).await
        }
        cli::Commands::Connect { action } => {
            commands::connect::run(action, env_override, output_fmt).await
        }
        cli::Commands::Certs { action } => {
            commands::certs::run(action, env_override, output_fmt).await
        }
        cli::Commands::Accounts { action } => {
            commands::accounts::run(action, env_override, output_fmt).await
        }
        cli::Commands::Orders { action } => {
            commands::orders::run(action, env_override, output_fmt).await
        }
        cli::Commands::Checkouts { action } => {
            commands::checkouts::run(action, env_override, output_fmt).await
        }
        cli::Commands::Plans { action } => {
            commands::plans::run(action, env_override, output_fmt).await
        }
        cli::Commands::Subscribers { action } => {
            commands::subscribers::run(action, env_override, output_fmt).await
        }
        cli::Commands::Subscriptions { action } => {
            commands::subscriptions::run(action, env_override, output_fmt).await
        }
        cli::Commands::Coupons { action } => {
            commands::coupons::run(action, env_override, output_fmt).await
        }
        cli::Commands::Invoices { action } => {
            commands::invoices::run(action, env_override, output_fmt).await
        }
        cli::Commands::Clubpag { action } => {
            commands::clubpag::run(action, env_override, output_fmt).await
        }
        cli::Commands::Webhooks { action } => match action {
            cli::WebhooksAction::Verify {
                token,
                signature,
                payload_file,
            } => {
                use std::io::Read;
                let payload = if let Some(path) = payload_file {
                    std::fs::read_to_string(&path)?
                } else {
                    let mut buf = String::new();
                    std::io::stdin().read_to_string(&mut buf)?;
                    buf
                };
                let mut hasher = sha2::Sha256::new();
                use sha2::Digest;
                hasher.update(token.as_bytes());
                hasher.update(b"-");
                hasher.update(payload.as_bytes());
                let computed = hasher
                    .finalize()
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>();
                if computed == signature {
                    output::print_success("Assinatura válida!");
                } else {
                    output::print_error("Assinatura inválida");
                    eprintln!("  Esperada: {signature}");
                    eprintln!("  Computada:  {computed}");
                    std::process::exit(1);
                }
                Ok(())
            }
        },
    };

    if let Err(e) = result {
        let msg = if let Some(pagbank_err) = e.downcast_ref::<pagbank_sdk::PagBankError>() {
            errors::translate(pagbank_err)
        } else {
            e.to_string()
        };
        output::print_error(&msg);
        std::process::exit(1);
    }

    Ok(())
}
