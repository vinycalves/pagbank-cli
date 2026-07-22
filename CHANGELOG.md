# Changelog

Todos os notáveis alterações neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [0.1.1] - 2026-07-22

### Added
- Testes unitários: Environment, Service, PagBankError, todos os modelos
- Testes unitários: PbConfig (CLI), serialização/desserialização TOML
- Mensagem de report de bugs em errors: reportar em https://github.com/vinycalves/pagbank-cli/issues
- Workflow CI (fmt, check, test, clippy) em push/PR
- Workflow release (crates.io, GitHub Release, AUR, Homebrew, .deb, .rpm)

### Fixed
- Compatibilidade sha2 v0.11 (LowerHex não implementado para GenericArray)
- Feature rustls-tls renomeada para rustls no reqwest 0.13
- Dependências pinadas para versões específicas

## [Unreleased]

### Added
- Workspace com dois crates: `pagbank-sdk` e `pb`
- SDK completa com HTTP client assíncrono (reqwest + tokio)
- Autenticação via Bearer token e OAuth2 (Connect)
- Geração automática de idempotency key (UUID v4)
- Tratamento de erros padronizado (`PagBankError`)
- Suporte a 3 serviços: Principal, Recorrência e Secure
- Suporte a 2 ambientes: Sandbox e Production
- Configuração via arquivo TOML (`~/.config/pb/config.toml`) e variáveis de ambiente
- CLI com 14 módulos de comandos:
  - `auth` — gerenciamento de autenticação
  - `config` — configurações do CLI
  - `keys` — chaves públicas
  - `connect` — OAuth2 e aplicações Connect
  - `certs` — certificados digitais (mTLS)
  - `accounts` — cadastro de contas
  - `orders` — pedidos, pagamentos, splits, taxas, armazenamento de cartão
  - `checkouts` — Checkouts PagBank
  - `plans` — planos de recorrência
  - `subscribers` — assinantes
  - `subscriptions` — assinaturas
  - `coupons` — cupons de desconto
  - `invoices` — faturas, pagamentos e estornos
  - `clubpag` — ClubPag (benefícios, cashback, cupons)
  - `webhooks` — verificação de autenticidade
- Output em tabela (comfy-table) e JSON
- Suporte a pipes e jq
- Flags globais: `-e`, `-o`, `--verbose`, `--config`

### SDK - Endpoints implementados

- **Orders**: create, get, list, pay, get_split, release_split
- **Charges**: capture, cancel, get_costs, store_card, create_3ds_session
- **Checkouts**: create, get, activate, deactivate
- **Plans**: create, get, list, update, activate, deactivate
- **Subscribers**: create, get, list, update, update_payment
- **Subscriptions**: create, get, list, update, cancel, suspend, activate, delete_coupons
- **Coupons**: create, get, list, activate, deactivate
- **Invoices**: get, list_payments, list_all_payments, create_refund, list_refunds, get_payment
- **Retries**: get, update, retry_now
- **Connect**: create_app, get_app, get_authorize_url, create_sms_auth, create_token, refresh_token, revoke_token
- **Accounts**: create, get
- **Public Keys**: create, get, update
- **Certificates**: create
- **Preferences**: get_preferences, update_preferences, get_encryption_keys, update_encryption_keys
- **ClubPag**: get_settings, update_settings, identify_purchase, list_benefits, redeem_benefit, get_cashback, list_coupons

### Models implementados

- Order, Charge, Plan, Subscriber, Subscription, Coupon
- Checkout, ClubPag (benefits, cashback, coupons, settings)
- Invoice, Retry
- Connect (app, token)
- Account, PublicKey, Certificate
- Common (Address, Amount, Item, Phone, QR Code, Shipping, Split, etc.)
