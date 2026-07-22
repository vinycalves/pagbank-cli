# pb — CLI PagBank

Uma ferramenta de linha de comando nativa em Rust que transforma as APIs do PagBank em comandos no seu terminal. Cem por cento código aberto.

## Filosofia

O PagBank expõe dezenas de endpoints REST — pedidos, pagamentos, recorrência, ClubPag, Connect OAuth2, chaves públicas, certificados digitais e mais. O `pb` empacota tudo isso numa única interface de terminal: rápida, tipada, sem dependências de runtime.

A arquitetura reflete dois públicos diferentes:

- **`pagbank-sdk`** — biblioteca Rust que qualquer outro projeto pode importar para consumir a API PagBank. Tipos fortemente tipados, tratamento de erros padronizado, idempotência automática.
- **`pb`** — binário CLI que consome a SDK. Escrito para o desenvolvedor que quer testar endpoints, automatizar tarefas ou integrar pagamentos em scripts shell.

## O que cobre

| Domínio | Comandos |
|---------|----------|
| Pedidos e Pagamentos | Criar, consultar, listar, pagar, capturar, cancelar, dividir |
| Checkouts | Criar, ativar, inativar |
| Recorrência | Planos, assinantes, assinaturas, cupons, faturas, estornos, retentativas |
| ClubPag | Configurações, benefícios, cashback, cupons |
| Connect (OAuth2) | Aplicações, autorização, tokens |
| Contas e Chaves | Cadastro, chaves públicas |
| Certificados | Criação de certificado digital (mTLS) |
| Webhooks | Verificação de autenticidade SHA-256 |

## Keywords

pagbank, pagseguro, CLI, pagamentos, boleto, pix, crédito, recorrência, assinaturas, fintech, rust

## Stack

- **Linguagem**: Rust (segurança de memória, zero custos de abstração)
- **HTTP**: `reqwest` com TLS nativo (rustls)
- **Async**: `tokio`
- **Serialização**: `serde` + `serde_json`
- **CLI**: `clap` (derive), `comfy-table` (tabelas), `dialoguer` (prompts interativos)
- **Distribuição**: crates.io, GitHub Releases, AUR, Homebrew, DEB, RPM

## Instalação

```
cargo install pb                        # crates.io
yay -S pagbank-cli-bin                   # AUR
brew install vinycius/tap/pb            # Homebrew
dpkg -i pb_0.1.0_amd64.deb              # Debian/Ubuntu
rpm -ivh pb-0.1.0-1.x86_64.rpm          # Fedora/RHEL
```

Ou baixe o binário direto da [última release](https://github.com/vinycius/pagbank-cli/releases/latest).

## Começar

```bash
pb config init
pb auth login --token SEU_TOKEN
pb orders create --item "Produto" --item-amount 1000 --method pix
```

Saída em tabela ou JSON (pipe para `jq`):

```bash
pb orders list --output json | jq '.[] | select(.status == "PAID")'
```

## Projeto

```
pagbank-cli/
├── sdk/         # pagbank-sdk — biblioteca reutilizável
├── cli/         # pb          — binário CLI
└── dist/        # pacotes AUR, Homebrew, etc.
```

## Licença

MIT
