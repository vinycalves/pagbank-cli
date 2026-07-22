# Convenções de Desenvolvimento

Padrões e convenções utilizados neste projeto. Seguir estas diretrizes mantém o código consistente e facilita a manutenção.

## Estrutura do Projeto

```
sdk/src/
├── client.rs          # PagBankClient - HTTP client central
├── config.rs          # Environment, Service, PagBankConfig
├── error.rs           # PagBankError enum
├── models/
│   ├── mod.rs         # parse_response, parse_list, parse_void
│   ├── common.rs      # Tipos compartilhados (Amount, Address, Phone, etc.)
│   ├── order.rs       # Order e tipos relacionados
│   └── ...
└── endpoints/
    ├── mod.rs         # Re-exports
    ├── orders.rs      # Funções CRUD para orders
    └── ...

cli/src/
├── main.rs            # Entry point, match nos comandos
├── cli.rs             # Clap definitions (enums de ação)
├── config.rs          # PbConfig (leitura/escrita do config.toml)
├── output.rs          # print_json, print_table, print_object_table
└── commands/
    ├── mod.rs         # Re-exports
    ├── orders.rs      # Implementação do comando orders
    └── ...
```

## Adicionando um Novo Endpoint

### 1. Criar o modelo (sdk/src/models/)

```rust
// sdk/src/models/meu_modelo.rs
use serde::{Deserialize, Serialize};
use super::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeuModelo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nome: Option<String>,
    // ... mais campos
}
```

**Regras:**
- Todos os campos Option com `skip_serializing_if = "Option::is_none"`
- Derivar `Debug, Clone, Serialize, Deserialize`
- Usar tipos do `common.rs` quando aplicável (Amount, Address, Phone, etc.)

### 2. Criar o endpoint (sdk/src/endpoints/)

```rust
// sdk/src/endpoints/meu_endpoint.rs
use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::meu_modelo::*;

pub async fn create(
    client: &PagBankClient,
    body: &serde_json::Value,
    opts: &RequestOptions,
) -> Result<MeuModelo, PagBankError> {
    let resp = client.post(Service::Main, "/meu-endpoint", body, opts).await?;
    crate::models::parse_response(resp).await
}

pub async fn get(client: &PagBankClient, id: &str) -> Result<MeuModelo, PagBankError> {
    let resp = client.get(Service::Main, &format!("/meu-endpoint/{id}")).await?;
    crate::models::parse_response(resp).await
}

pub async fn list(
    client: &PagBankClient,
    params: &[(String, String)],
) -> Result<Vec<MeuModelo>, PagBankError> {
    let query: String = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let path = if query.is_empty() {
        "/meu-endpoint".to_string()
    } else {
        format!("/meu-endpoint?{query}")
    };
    let resp = client.get(Service::Main, &path).await?;
    crate::models::parse_list(resp).await
}
```

**Regras:**
- Usar `Service::Main`, `Service::Recurring` ou `Service::Secure` conforme o endpoint
- Retornar `Result<T, PagBankError>`
- Usar `parse_response` para objetos, `parse_list` para listas, `parse_void` para void
- Para endpoints com query params, usar o padrão `params: &[(String, String)]`

### 3. Registrar no mod.rs

```rust
// sdk/src/models/mod.rs - adicionar:
pub mod meu_modelo;
pub use meu_modelo::*;

// sdk/src/endpoints/mod.rs - adicionar:
pub mod meu_endpoint;
```

### 4. Criar a action no CLI (cli/src/cli.rs)

```rust
// Adicionar variante no enum Commands:
#[command(about = "Descrição do comando")]
MeuComando {
    #[command(subcommand)]
    action: MeuComandoAction,
},

// Criar o enum de ação:
#[derive(Subcommand)]
pub enum MeuComandoAction {
    #[command(about = "Criar item")]
    Create {
        #[arg(long)]
        nome: String,
        // ...
    },
    #[command(about = "Consultar item")]
    Get { id: String },
    #[command(about = "Listar itens")]
    List {
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
}
```

**Regras para flags:**
- Sempre usar `--long` (nunca positional args para dados importantes)
- Valores padrão com `default_value`
- Opções com `Option<T>`
- Descriptions em português

### 5. Implementar o comando (cli/src/commands/)

```rust
// cli/src/commands/meu_comando.rs
use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};

use crate::cli::MeuComandoAction;
use crate::config::PbConfig;
use crate::output;

fn make_client(config: &PbConfig, env_override: Option<&str>) -> Result<PagBankClient> {
    let active = config.get_active_config(env_override);
    let pagbank_config = PagBankConfig {
        environment: active.environment.parse().unwrap_or(Environment::Sandbox),
        token: active.token.clone(),
        recurring_token: active.recurring_token.clone(),
        client_id: active.client_id.clone(),
        client_secret: active.client_secret.clone(),
    };
    Ok(PagBankClient::new(pagbank_config))
}

pub async fn run(
    action: MeuComandoAction,
    env_override: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) -> Result<()> {
    let config = PbConfig::load()?;
    let client = make_client(&config, env_override)?;

    match action {
        MeuComandoAction::Create { nome, .. } => {
            let body = serde_json::json!({ "nome": nome });
            let result = pagbank_sdk::endpoints::meu_endpoint::create(&client, &body, &Default::default()).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Item Criado", &val),
            }
            Ok(())
        }
        MeuComandoAction::Get { id } => {
            let result = pagbank_sdk::endpoints::meu_endpoint::get(&client, &id).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => output::print_object_table("Item", &val),
            }
            Ok(())
        }
        MeuComandoAction::List { page, per_page } => {
            let params = vec![
                ("page".to_string(), page.to_string()),
                ("per_page".to_string(), per_page.to_string()),
            ];
            let result = pagbank_sdk::endpoints::meu_endpoint::list(&client, &params).await?;
            let val = serde_json::to_value(result)?;
            match output_fmt {
                crate::cli::OutputFormat::Json => output::print_json(&val),
                crate::cli::OutputFormat::Table => {
                    if let Some(arr) = val.as_array() {
                        let rows: Vec<Vec<String>> = arr.iter().map(|o| {
                            vec![
                                o["id"].as_str().unwrap_or("").to_string(),
                                o["nome"].as_str().unwrap_or("").to_string(),
                            ]
                        }).collect();
                        output::print_table(&["ID", "Nome"], rows);
                    }
                }
            }
            Ok(())
        }
    }
}
```

### 6. Registrar no main.rs

```rust
// cli/src/main.rs - adicionar no match:
cli::Commands::MeuComando { action } => {
    commands::meu_comando::run(action, env_override, output_fmt).await
}
```

## Padrões de Código

### Tratamento de Erros

- SDK: usar `PagBankError` (thiserror)
- CLI: usar `anyhow::Result` e `?` operator
- Erros de API: `PagBankError::Api { status, code, message }`
- Tokens ausentes: `PagBankError::NoToken` / `PagBankError::NoRecurringToken`

### Output

```python
# Para objetos:
match output_fmt {
    OutputFormat::Json => output::print_json(&val),
    OutputFormat::Table => output::print_object_table("Título", &val),
}

# Para listas:
match output_fmt {
    OutputFormat::Json => output::print_json(&val),
    OutputFormat::Table => {
        if let Some(arr) = val.as_array() {
            let rows: Vec<Vec<String>> = arr.iter().map(|o| {
                vec![o["field"].as_str().unwrap_or("").to_string()]
            }).collect();
            output::print_table(&["Header"], rows);
        }
    }
}

# Para operações void:
output::print_success("Operação realizada com sucesso");
```

### Serviços

| Serviço | Uso |
|---------|-----|
| `Service::Main` | Orders, Charges, Checkouts, Accounts, Keys, etc. |
| `Service::Recurring` | Plans, Subscribers, Subscriptions, Coupons, Invoices |
| `Service::Secure` | Connect (OAuth2), Certificates |

### Serialização JSON

- Usar `serde_json::json!()` para construir bodies
- Nunca construir JSON manualmente com strings
- Campos opcionais: `skip_serializing_if = "Option::is_none"`

### Naming Conventions

- **SDK**: `snake_case` para funções e variáveis, `PascalCase` para structs/enums
- **CLI**: comandos em `kebab-case` (`split-release`), ações em `PascalCase`
- **Arquivos**: `snake_case.rs`

### Imports

```rust
// SDK
use crate::client::{PagBankClient, RequestOptions};
use crate::config::Service;
use crate::error::PagBankError;
use crate::models::order::*;

// CLI
use anyhow::Result;
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment};
use crate::cli::OrdersAction;
use crate::config::PbConfig;
use crate::output;
```

## Testes

```bash
# Compilar sem erros
cargo check

# Rodar todos os testes
cargo test

# Rodar testes de um crate específico
cargo test -p pagbank-sdk
cargo test -p pb
```

## Formatação

```bash
# Formatar código
cargo fmt

# Verificar sem buildar
cargo fmt -- --check
```

## Lint

```bash
# Clippy
cargo clippy

# Com warnings como erros
cargo clippy -- -D warnings
```
