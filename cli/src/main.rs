mod cli;
mod commands;
mod config;
mod errors;
mod output;
mod pix;

use clap::CommandFactory;
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
        cli::Commands::Preferences { action } => {
            commands::preferences::run(action, env_override, output_fmt).await
        }
        cli::Commands::Retries { action } => {
            commands::retries::run(action, env_override, output_fmt).await
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
                if constant_time_eq(&computed, &signature) {
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
        cli::Commands::Completion { shell } => {
            generate_completion(shell);
            Ok(())
        }
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

fn constant_time_eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn generate_completion(shell: cli::Shell) {
    use clap_complete::{generate, Shell};
    let mut cmd = cli::Cli::command();
    let shell = match shell {
        cli::Shell::Bash => Shell::Bash,
        cli::Shell::Zsh => Shell::Zsh,
        cli::Shell::Fish => Shell::Fish,
        cli::Shell::Powershell => Shell::PowerShell,
        cli::Shell::Elvish => Shell::Elvish,
    };
    generate(shell, &mut cmd, "pb", &mut std::io::stdout());
}
