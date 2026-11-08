# pagbank-sdk

<p align="center">
  <strong>SDK Rust para as APIs do PagBank</strong>
  <br>
  Tipos fortemente tipados, tratamento de erros padronizado, idempotência automática.
</p>

<hr>

## Sobre

`pagbank-sdk` é uma biblioteca Rust que consome todas as APIs REST do PagBank
de forma segura e tipada. Ela abstrai:

- Autenticação via Bearer token, Basic auth (EDI) e OAuth2 (Connect)
- Geração automática de idempotency key (UUID v4)
- Serviços Principal, Recorrência, Secure, SDK, Transferência e EDI
- Dois ambientes: Sandbox e Production
- Tratamento de erros unificado (`PagBankError`)

> Esta SDK é consumida pelo CLI [`pb`](https://crates.io/crates/pb),
> mas pode ser usada independentemente em qualquer projeto Rust.

## Instalação

```toml
[dependencies]
pagbank-sdk = "0.1"
```

## Uso básico

```rust
use pagbank_sdk::{PagBankClient, PagBankConfig, Environment, Service, PagBankError};

let config = PagBankConfig {
    environment: Environment::Sandbox,
    token: "seu-token-aqui".to_string(),
    recurring_token: None,
    transfer_token: None,
    edi_user: None,
    edi_token: None,
    client_id: None,
    client_secret: None,
};
let client = PagBankClient::new(config);

// Criar um pedido
let body = serde_json::json!({
    "reference_id": "pedido-001",
    "customer": {
        "name": "João",
        "email": "joao@test.com",
        "tax_id": "12345678909"
    },
    "items": [{
        "name": "Produto",
        "quantity": 1,
        "unit_amount": 1000
    }],
    "charges": [{
        "amount": { "value": 1000, "currency": "BRL" },
        "payment_method": { "type": "PIX" }
    }]
});
let order = pagbank_sdk::endpoints::orders::create(&client, &body, &Default::default()).await?;
println!("Pedido criado: {}", order.id.unwrap_or_default());
```

## Módulos

### Config

```rust
use pagbank_sdk::{Environment, Service, PagBankConfig};
```

| Tipo | Descrição |
|------|-----------|
| `Environment` | Sandbox ou Production |
| `Service` | Main, Recurring, Secure, Sdk, Transfer ou Edi |
| `PagBankConfig` | Token, recurring_token, transfer_token, edi_user, edi_token, client_id, client_secret |

```rust
// Resolver URL base para cada serviço + ambiente
Service::Main.base_url(&Environment::Sandbox);
// → "https://sandbox.api.pagseguro.com"
```

### Client

```rust
use pagbank_sdk::PagBankClient;

let client = PagBankClient::new(config);
```

Métodos HTTP:
- `get(service, path)` → GET request com Bearer token
- `post(service, path, body, options)` → POST com idempotency key automática
- `post_multipart(service, path, form, options)` → POST multipart (upload de PDF de disputa)
- `put(service, path, body, options)` → PUT com idempotency key
- `patch(service, path, body, options)` → PATCH com idempotency key
- `delete(service, path)` → DELETE request
- `get_edi(path)` → GET na API EDI com Basic auth (`edi_user:edi_token`)

O `RequestOptions` permite sobrescrever a idempotency key:

```rust
use pagbank_sdk::client::RequestOptions;

let opts = RequestOptions {
    idempotency_key: Some("meu-idempotency-key".to_string()),
};
```

### Error Handling

```rust
use pagbank_sdk::PagBankError;
```

| Variant | Descrição |
|---------|-----------|
| `Api { status, code, message }` | Erro da API PagBank (JSON estruturado) |
| `ApiRaw { status, body }` | Erro da API sem formato JSON |
| `Network(reqwest::Error)` | Erro de rede (timeout, DNS, etc.) |
| `Serialization(serde_json::Error)` | Erro ao serializar/desserializar JSON |
| `Url(url::ParseError)` | URL inválida |
| `NoToken` | Token de autenticação não configurado |
| `NoRecurringToken` | Token de recorrência não configurado |
| `NoTransferToken` | Token de transferência não configurado |
| `NoEdiCredentials` | Credenciais EDI (`edi_user`/`edi_token`) não configuradas |
| `InvalidIdempotencyKey(String)` | Idempotency key em formato inválido |
| `Auth(String)` | Erro de autenticação genérico |
| `Other(String)` | Outro erro |

Implementa `std::error::Error` e `Display`.

### Models

Todos os objetos da API PagBank como structs Rust tipadas com Serde:

```rust
use pagbank_sdk::models::*;
```

| Módulo | Modelos |
|--------|---------|
| `common` | Amount, Address, Phone, Item, Shipping, Split, QrCode, Link, PaginationParams |
| `order` | Order, OrderCustomer, CreateOrderRequest, SubMerchant, Wallet |
| `charge` | Charge, PaymentMethod, Card, Boleto, Pix, Fine, Interest, Discount, TokenData, AuthenticationMethod, Recurring, ChargeFees |
| `chargeback` | Chargeback, ChargebackTransaction, Dispute, DisputeFile |
| `plan` | Plan, PlanAmount, PlanInterval, PlanTrial, CreatePlanRequest |
| `subscriber` | Subscriber, SubscriberPaymentMethod, CreateSubscriberRequest |
| `subscription` | Subscription, SubscriptionCycles, CreateSubscriptionRequest |
| `coupon` | Coupon, CouponDiscount, CreateCouponRequest |
| `invoice` | Invoice, InvoicePayment, Refund, Retry |
| `notification` | NotificationPreference |
| `transfer` | Transfer, TransferAmount, TransferInstrument, TransferP2p, TransferPix |
| `edi` | EdiMovements, EdiPagination |
| `checkout` | Checkout, CreateCheckoutRequest |
| `clubpag` | ClubPagSettings, ClubPagBenefit, ClubPagCashback, ClubPagCoupon |
| `connect` | ConnectApp, ConnectToken, CreateAppRequest |
| `account` | Account, CreateAccountRequest |
| `public_key` | PublicKey, CreatePublicKeyRequest |
| `certificate` | Certificate, CreateCertificateRequest |

Funções auxiliares de parse:
- `parse_response<T>(resp)` → parse de objeto único
- `parse_list<T>(resp)` → parse de lista (suporta chaves `data`, `orders`, `items`, `payments`, `refunds`, `preferences`, `transfers`)
- `parse_void(resp)` → resposta sem corpo

### Endpoints

Cada módulo da API tem funções CRUD:

```rust
use pagbank_sdk::endpoints;
```

#### Pedidos
| Função | HTTP | Caminho |
|--------|------|---------|
| `orders::create(client, body, opts)` | POST | `/orders` |
| `orders::get(client, id)` | GET | `/orders/{id}` |
| `orders::list(client, params)` | GET | `/orders?{params}` |
| `orders::pay(client, id, body)` | POST | `/orders/{id}/pay` |
| `orders::get_split(client, id)` | GET | `/orders/{id}/splits` |
| `orders::release_split(client, id)` | POST | `/orders/{id}/splits/release` |

#### Cobranças
| Função | HTTP | Caminho |
|--------|------|---------|
| `charges::capture(client, id, body)` | POST | `/charges/{id}/capture` |
| `charges::cancel(client, id)` | POST | `/charges/{id}/cancel` |
| `charges::get_costs(client, id)` | GET | `/charges/{id}/costs` |
| `charges::store_card(client, body)` | POST | `/cards` |
| `charges::create_3ds_session(client)` | POST | `/checkout-sdk/sessions` |

#### Recorrência
| Função | HTTP | Caminho |
|--------|------|---------|
| `plans::create(client, body)` | POST | `/plans` |
| `plans::get(client, id)` | GET | `/plans/{id}` |
| `plans::list(client, params)` | GET | `/plans` |
| `plans::update(client, id, body)` | PUT | `/plans/{id}` |
| `plans::activate(client, id)` | PUT | `/plans/{id}/activate` |
| `plans::deactivate(client, id)` | PUT | `/plans/{id}/deactivate` |
| `subscribers::create(...)` | POST | `/customers` |
| `subscribers::get(...)` | GET | `/customers/{id}` |
| `subscribers::list(...)` | GET | `/customers` |
| `subscriptions::create(...)` | POST | `/subscriptions` |
| `subscriptions::get(...)` | GET | `/subscriptions/{id}` |
| `subscriptions::list(...)` | GET | `/subscriptions` |
| `coupons::create(...)` | POST | `/coupons` |
| `coupons::list(...)` | GET | `/coupons` |
| `invoices::get(...)` | GET | `/invoices/{id}` |
| `invoices::create_refund(...)` | POST | `/payments/{id}/refund` |
| `retries::get(...)` | GET | `/retries/{id}` |
| `retries::retry_now(...)` | PUT | `/retries/{id}/retry` |
| `preferences::get_preferences(...)` | GET | `/notification-preferences` |
| `preferences::update_preferences(...)` | PUT | `/notification-preferences` |
| `preferences::get_encryption_key(...)` | GET | `/encryption-key` |
| `preferences::create_encryption_key(...)` | PUT | `/encryption-key` |

#### Checkout, Accounts, Connect, ClubPag
| Função | HTTP | Caminho |
|--------|------|---------|
| `checkouts::create(client, body)` | POST | `/checkouts` |
| `checkouts::get(client, id)` | GET | `/checkouts/{id}` |
| `checkouts::activate(client, id)` | POST | `/checkouts/{id}/activate` |
| `checkouts::deactivate(client, id)` | POST | `/checkouts/{id}/deactivate` |
| `accounts::create(client, body)` | POST | `/accounts` |
| `accounts::get(client, id)` | GET | `/accounts/{id}` |
| `connect::create_app(client, body)` | POST | `/oauth2/application` |
| `connect::get_app(client, id)` | GET | `/oauth2/application/{id}` |
| `connect::create_token(client, body)` | POST | `/oauth2/token` |
| `connect::refresh_token(client, body)` | POST | `/oauth2/token/refresh` |
| `connect::revoke_token(client, body)` | POST | `/oauth2/token/revoke` |
| `public_keys::create(client, type)` | POST | `/public-keys` |
| `public_keys::get(client, id)` | GET | `/public-keys/{id}` |
| `public_keys::update(client, id, body)` | PUT | `/public-keys/{id}` |
| `certificates::create(client, body)` | POST | `/certificates` |
| `clubpag::get_settings(client)` | GET | `/clubpag/settings` |
| `clubpag::list_benefits(client)` | GET | `/clubpag/benefits` |
| `clubpag::get_cashback(client)` | GET | `/clubpag/cashback` |
| `clubpag::list_coupons(client)` | GET | `/clubpag/coupons` |

#### Chargebacks, Notificações, Transferências e EDI
| Função | HTTP | Caminho |
|--------|------|---------|
| `chargebacks::get(client, id)` | GET | `/chargebacks/{id}` |
| `chargebacks::create_dispute(client, id, file_name, bytes)` | POST | `/chargebacks/{id}/disputes` |
| `chargebacks::get_dispute(client, cbk_id, disp_id)` | GET | `/chargebacks/{cbk_id}/disputes/{disp_id}` |
| `notifications::create(client, body)` | POST | `/notifications/preferences` |
| `notifications::get(client, id)` | GET | `/notifications/preferences/{id}` |
| `notifications::list(client)` | GET | `/notifications/preferences` |
| `notifications::update(client, id, body)` | PATCH | `/notifications/preferences/{id}` |
| `notifications::delete(client, id)` | DELETE | `/notifications/preferences/{id}` |
| `transfers::create(client, body, opts)` | POST | `/transfers` |
| `transfers::get(client, id)` | GET | `/transfers/{id}` |
| `transfers::list(client, params)` | GET | `/transfers?{params}` |
| `edi::movements(client, type, date, page, page_size)` | GET | `/movement/v3.00/{type}/{date}` |
| `invoices::get_refund(client, id)` | GET | `/refunds/{id}` |

> Transferências exigem certificado digital e um `transfer_token` obtido no
> Connect (escopos `transfer.create` e/ou `transfer.read`). O EDI usa
> credenciais próprias (`edi_user` = nº do estabelecimento e `edi_token`) via
> Basic auth, não possui sandbox e retorna dados de D-1.

## Tratamento de Erros

```rust
match result {
    Ok(order) => println!("Pedido: {:?}", order),
    Err(PagBankError::Api { status, code, message }) => {
        eprintln!("Erro PagBank [{}] {}: {}", status, code, message);
    }
    Err(PagBankError::NoToken) => {
        eprintln!("Token não configurado");
    }
    Err(e) => eprintln!("Erro: {}", e),
}
```

## Testes

```bash
cargo test -p pagbank-sdk
```

## CLI

O binário [`pb`](https://crates.io/crates/pb) consome esta SDK e expõe todos
os endpoints como comandos de terminal. Instale com:

```bash
cargo install pagbank-cli
```

## Licença

MIT
