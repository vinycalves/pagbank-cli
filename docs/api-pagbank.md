# API PagBank — Mapeamento de Conceitos

Este documento mapeia como a API do PagBank funciona, as relações entre os
endpoints e como o CLI `pb` os implementa.

## Arquitetura Geral

A API PagBank segue o padrão RESTful, com respostas em JSON. Quatro domínios
principais coexistem:

```
PagBank API
├── Pagamento (pagamentos avulsos)
│   ├── Account        ← conta de recebedor (seller)
│   ├── Order + Charge ← pedido + cobrança
│   │   ├── QR Code (PIX)
│   │   ├── Deep Link (Pagar com PagBank via app)
│   │   ├── SDWO (carteiras digitais escalonadas)
│   │   ├── Apple Pay / Google Pay (wallets)
│   │   ├── Sub-merchant (facilitadores de pagamento)
│   │   └── Divisão de pagamento (split)
│   ├── Checkout       ← página de pagamento simplificada
│   ├── Public Key     ← criptografia de cartão
│   ├── Connect        ← OAuth2 (agir em nome de outro usuário)
│   ├── Certificate    ← mTLS (segurança extra)
│   ├── 3DS Session    ← autenticação 3D Secure
│   ├── Chargeback     ← contestação + disputa (PDF)
│   └── Transfer       ← P2P entre contas PagBank e PIX (exige certificado)
│
├── Notificação (webhooks)
│   └── Preference     ← eventos CHARGEBACK, ENROLLMENT, CANCELLATION, ORDER.CHARGE
│
├── Recorrência (assinaturas)
│   ├── Plan           ← modelo de cobrança
│   ├── Subscriber     ← cliente da recorrência
│   ├── Subscription   ← vínculo plano + assinante
│   │   ├── Coupons    ← cupons de desconto na assinatura
│   │   └── Delete     ← remover cupons da assinatura
│   ├── Invoice        ← fatura gerada
│   ├── Payment        ← pagamento da fatura
│   ├── Refund         ← estorno (criar, listar e consultar)
│   ├── Coupon         ← desconto (avulso)
│   ├── Retry          ← retentativa de cobrança
│   ├── Preferences    ← notificações e chave de criptografia
│   └── Encryption Key ← chave pública para criptografia
│
├── ClubPag (fidelidade)
│   └── ClubPag        ← cashback, benefícios, cupons
│
├── SDK (serviços auxiliares)
│   └── Checkout SDK   ← sessão 3DS, simulação de pagamento
│
└── EDI (conciliação)
    └── Movement       ← transacional, financeiro, cashouts, saldos (D-1)
```

---

## Fluxo dos Pagamentos (Ordens)

O conceito central é **Order** (pedido). Toda transação começa com um pedido.

```
Order
├── Contém: customer, items, charges[], qr_codes[]
├── Pode ser criada sem pagamento (WAITING)
└── Paga via POST /orders/{id}/pay

Charge (dentro de Order)
├── Contém: amount, payment_method, status
├── Pode ser capturada (autorização → captura)
├── Pode ser cancelada
└── Pode ter split (divisão entre múltiplos recebedores)

Links entre Order e Account
├── Order.customer = dados do comprador
├── Split nos charges → direciona valores para Accounts (sellers)
└── Account precisa ser criada antes do split
```

### Passo a passo de um pedido PIX

1. Criar uma `Account` para cada recebedor (se for usar split)
2. Criar `Order` com `payment_method.type = "PIX"` e `qr_codes`
3. API retorna QR code PIX (base64)
4. Comprador paga escaneando o QR code
5. Webhook notifica o pagamento (ou consulta via GET)

### Passo a passo de um pedido com cartão

1. Opcional: criar `PublicKey` para criptografar dados do cartão
2. Criar `Order` sem pagamento
3. Pagar via `POST /orders/{id}/pay` com dados do cartão
4. Capturar se for autorização em 2 etapas

---

## Fluxo da Recorrência (Assinaturas)

```
Plan (modelo)
├── Define: valor, período, trial, ciclos
└── Ativo/Inativo

Subscriber (cliente)
├── Dados: nome, email, CPF, telefone, cartão
└── Criado antes da assinatura

Subscription (vínculo)
├── plan_id + subscriber_id
├── Gera Invoices (faturas) automaticamente
├── Pode ter Coupons (descontos)
└── Status: ACTIVE, SUSPENDED, CANCELED

Invoice (fatura)
├── Gerada a cada ciclo da subscription
├── Tem payments (tentativas de cobrança)
└── Pode ser estornada (refund)

Retry (retentativa)
├── Se o pagamento de uma invoice falha
├── Configurável: quantas vezes, intervalo
└── Pode ser disparado manualmente

Coupon (desconto)
├── Percentual ou fixo
├── Aplicado na subscription
└── Ativo/Inativo
```

### Fluxo completo

```
1. Cria Plano (monthly, R$ 49,90)
2. Cria Assinante (João, joao@email.com)
3. Cria Assinatura (Plano A + Assinante João)
4. A cada ciclo:
   ├── Invoice gerada automaticamente
   ├── Tentativa de cobrança (charge)
   ├── Se falhar → Retry agenda nova tentativa
   └── Se sucesso → fatura marcada como PAID
5. Pode cancelar/suspender a assinatura
```

---

## Connect (OAuth2)

Permite que uma aplicação aja em nome de outra conta PagBank.

```
Aplicação (App)
├── Cadastrada via POST /oauth2/application
├── Gera client_id + client_secret
└── Define redirect_uri (para onde o usuário volta)

Fluxo
1. App é criada
2. Usuário autoriza via URL (GET /oauth2/authorize)
3. Usuário volta para redirect_uri com ?code=XXXX
4. Aplicação troca code por access_token (POST /oauth2/token)
5. Token pode ser renovado (refresh) ou revogado
```

---

## ClubPag (Fidelidade)

Programa de cashback e benefícios.

```
Configurações
├── Ativar/desativar o ClubPag
└── Definir regras

Compra
├── Identificada via POST /clubpag/purchase
├── Gera cashback (percentual do valor)
└── Cashback consultável via GET /clubpag/cashback

Benefícios
├── Lista de benefícios disponíveis
├── Resgate via POST /clubpag/benefits/redeem
└── Cada benefício tem um ID

Cupons
├── Cupons exclusivos ClubPag
└── Listados via GET /clubpag/coupons
```

---

## Chaves Públicas e Criptografia

Usadas para criptografar dados de cartão no frontend (checkout transparente).

```
1. POST /public-keys → gera par de chaves
2. Retorna: { id, public_key, status }
3. Usa a chave pública para criptografar card.number no frontend
4. Envia o card.number criptografado na criação do pedido
5. PagBank descriptografa no backend

Tipo: CARD (cartão), WEB, ANDROID, IOS
```

---

## Certificado Digital (mTLS)

Camada extra de segurança para chamadas à API.

```
POST /certificates
├── body: { certificate: "base64 do .p12", password: "xxx" }
└── Retorna: { id, status, created_at }
```

---

## Webhooks

Notificações enviadas pelo PagBank sobre eventos.

```
Eventos comuns
├── ORDER.PAID        → pedido pago
├── ORDER.CANCELED    → pedido cancelado
├── CHARGE.CAPTURED   → cobrança capturada
├── CHARGE.CANCELED   → cobrança cancelada
├── SUBSCRIPTION.CANCELED → assinatura cancelada
└── CLUBPAG.PURCHASE  → compra ClubPag

Verificação de autenticidade
├── Header: x-authenticity-token
├── Algoritmo: SHA-256("{token}-{payload_raw}")
└── Comando: pb webhooks verify --token X --signature Y payload.json
```

---

## Chargebacks e Disputas

Quando o portador contesta uma transação, o PagBank abre um chargeback. O lojista
pode acompanhar o caso e enviar uma defesa (disputa) em PDF.

```
Chargeback
├── Status: AWAITING_EVIDENCE, REVIEW_WITH_PAGBANK, SUBMITTED_ISSUER,
│           APPROVED, DENIED, EXPIRED, CLOSED, WON, LOST
├── classification: NOT_RECOGNIZED, DISAGREEMENT, REPORT
├── due_date: prazo para enviar a defesa
└── transaction: dados da venda original (cartão mascarado, NSU, TID, ARN)

Disputa
├── Só pode ser criada para chargeback em AWAITING_EVIDENCE
├── Um único PDF por chargeback (máx. 4 MB e 18 páginas)
├── Status: RECEIVED, ACCEPTED, REJECTED
└── Consulta: GET /chargebacks/{cbk_id}/disputes/{disp_id}
```

| Comando CLI | Endpoint |
|-------------|----------|
| `pb chargebacks get <id>` | `GET /chargebacks/{id}` |
| `pb chargebacks dispute <id> --file defesa.pdf` | `POST /chargebacks/{id}/disputes` |
| `pb chargebacks get-dispute <cbk_id> <disp_id>` | `GET /chargebacks/{cbk_id}/disputes/{disp_id}` |

---

## API de Notificação (webhooks)

API separada das preferências de recorrência (`/notification-preferences`).
Cada preferência associa um ou mais eventos a uma URL de destino.

```
Eventos
├── CHARGEBACK    → abertura/atualização de chargeback
├── ENROLLMENT    → cadastro/credenciamento
├── CANCELLATION  → cancelamentos
└── ORDER.CHARGE  → cobranças de pedidos

CRUD
├── POST   /notifications/preferences
├── GET    /notifications/preferences
├── GET    /notifications/preferences/{pref_id}
├── PATCH  /notifications/preferences/{pref_id}
└── DELETE /notifications/preferences/{pref_id}
```

> Antes de criar a preferência, crie uma chave pública do tipo `webhook`
> (`pb keys create --type WEB`) para validar a autenticidade das notificações.

| Comando CLI | Endpoint |
|-------------|----------|
| `pb notifications create --events CHARGEBACK --target URL` | `POST /notifications/preferences` |
| `pb notifications get <id>` | `GET /notifications/preferences/{id}` |
| `pb notifications list` | `GET /notifications/preferences` |
| `pb notifications update <id> --events ... --target URL` | `PATCH /notifications/preferences/{id}` |
| `pb notifications delete <id>` | `DELETE /notifications/preferences/{id}` |

---

## Transferências

Movimenta saldo entre contas PagBank (P2P) ou para outros bancos (PIX).

```
Pré-requisitos
├── Certificado digital (mTLS) criado
├── Aplicação Connect criada (client_id + client_secret)
└── access_token com escopo transfer.create e/ou transfer.read

P2P
├── Destino por account_id ou bank_branch + account_number (ou e-mail)
└── Status: CREATED → PROCESSING → SUCCESS | FAILED

PIX
├── Destino por chave PIX ou dados bancários
│   (tax_id, bank_ispb, bank_branch, account_number, account_type)
└── Requer liberação do time comercial do PagBank

Notificação
└── notification_urls recebe o resultado final da transferência
```

| Comando CLI | Endpoint |
|-------------|----------|
| `pb transfers create --amount 150099 --type P2P ...` | `POST /transfers` |
| `pb transfers get <id>` | `GET /transfers/{id}` |
| `pb transfers list --reference-id ... --start-date ...` | `GET /transfers?reference_id=&start_date=&end_date=` |

> O token de transferência é diferente do token principal. Configure-o com
> `pb config set transfer_token <TOKEN>`. A API fica em
> `secure.api.pagseguro.com` e exige certificado digital.

---

## EDI (conciliação)

Extrato eletrônico do dia anterior (D-1), com autenticação Basic
(`edi_user` = número do estabelecimento e `edi_token`). Não existe sandbox.

```
GET https://edi.api.pagbank.com.br/movement/v3.00/{tipo}/{AAAA-MM-DD}
    ?pageNumber=1&pageSize=1000

Tipos
├── transactional → vendas, chargebacks, cancelamentos, ajustes e split
├── financial     → liquidações, pagamentos, cashouts e antecipações
├── cashouts      → saques para banco, PagBank ou saldo carteira
└── balances      → saldos

Resposta
├── detalhes[]    → linhas do extrato (campos variam por tipo)
├── pagination    → elements, totalPages, page, totalElements
└── header VALIDADO → true quando o dia está completo
```

| Comando CLI | Endpoint |
|-------------|----------|
| `pb edi movements --type transactional --date 2026-11-06` | `GET /movement/v3.00/transactional/2026-11-06` |

---

## Mapeamento CLI → API

| Comando CLI | Endpoint | Descrição |
|------------|----------|-----------|
| `pb keys create --type CARD` | `POST /public-keys` | Criar chave pública |
| `pb keys get <id>` | `GET /public-keys/{id}` | Consultar chave |
| `pb keys update <id>` | `PUT /public-keys/{id}` | Alterar chave |
| `pb connect app-create` | `POST /oauth2/application` | Criar app Connect |
| `pb connect app-get <id>` | `GET /oauth2/application/{id}` | Consultar app |
| `pb connect authorize` | `GET /oauth2/authorize` | Gerar URL de autorização |
| `pb connect authorize-sms` | `POST /oauth2/authorize/sms` | Autorizar via SMS |
| `pb connect token` | `POST /oauth2/token` | Obter access token |
| `pb connect token-refresh` | `POST /oauth2/token/refresh` | Renovar token |
| `pb connect token-revoke` | `POST /oauth2/token/revoke` | Revogar token |
| `pb certs create` | `POST /certificates` | Criar certificado mTLS |
| `pb accounts create` | `POST /accounts` | Criar conta (seller) |
| `pb accounts get <id>` | `GET /accounts/{id}` | Consultar conta |
| `pb orders create` | `POST /orders` | Criar pedido (pix, cartão, etc.) |
| `pb orders get <id>` | `GET /orders/{id}` | Consultar pedido |
| `pb orders list` | `GET /orders?charge_id=` | Listar pedidos |
| `pb orders pay <id>` | `POST /orders/{id}/pay` | Pagar pedido |
| `pb orders split <id>` | `GET /orders/{id}/splits` | Consultar split |
| `pb orders split-release <id>` | `POST /orders/{id}/splits/release` | Liberar custódia |
| `pb orders capture <id>` | `POST /charges/{id}/capture` | Capturar |
| `pb orders cancel <id>` | `POST /charges/{id}/cancel` | Cancelar |
| `pb orders fees <id>` | `GET /charges/{id}/costs` | Consultar taxas |
| `pb orders card-store` | `POST /cards` | Armazenar cartão |
| `pb orders 3ds-session` | `POST /checkout-sdk/sessions` | Criar sessão 3DS |
| `pb checkouts create` | `POST /checkouts` | Criar checkout |
| `pb checkouts get <id>` | `GET /checkouts/{id}` | Consultar checkout |
| `pb checkouts activate` | `POST /checkouts/{id}/activate` | Ativar checkout |
| `pb checkouts deactivate` | `POST /checkouts/{id}/deactivate` | Inativar checkout |
| `pb plans create` | `POST /plans` | Criar plano |
| `pb plans get <id>` | `GET /plans/{id}` | Consultar plano |
| `pb plans list` | `GET /plans` | Listar planos |
| `pb plans update <id>` | `PUT /plans/{id}` | Alterar plano |
| `pb plans activate <id>` | `PUT /plans/{id}/activate` | Ativar plano |
| `pb plans deactivate <id>` | `PUT /plans/{id}/deactivate` | Inativar plano |
| `pb subscribers create` | `POST /customers` | Criar assinante |
| `pb subscribers get <id>` | `GET /customers/{id}` | Consultar assinante |
| `pb subscribers list` | `GET /customers` | Listar assinantes |
| `pb subscribers update-profile` | `PUT /customers/{id}` | Alterar dados |
| `pb subscribers update-payment` | `PUT /customers/{id}/payment-method` | Alterar cartão |
| `pb subscriptions create` | `POST /subscriptions` | Criar assinatura |
| `pb subscriptions get <id>` | `GET /subscriptions/{id}` | Consultar assinatura |
| `pb subscriptions list` | `GET /subscriptions` | Listar assinaturas |
| `pb subscriptions update <id>` | `PUT /subscriptions/{id}` | Alterar assinatura |
| `pb subscriptions cancel <id>` | `PUT /subscriptions/{id}/cancel` | Cancelar |
| `pb subscriptions suspend <id>` | `PUT /subscriptions/{id}/suspend` | Suspender |
| `pb subscriptions activate <id>` | `PUT /subscriptions/{id}/activate` | Ativar |
| `pb subscriptions invoices <id>` | `GET /subscriptions/{id}/invoices` | Listar faturas |
| `pb subscriptions delete-coupons <id>` | `DELETE /subscriptions/{id}/coupons` | Remover cupons |
| `pb coupons create` | `POST /coupons` | Criar cupom |
| `pb coupons get <id>` | `GET /coupons/{id}` | Consultar cupom |
| `pb coupons list` | `GET /coupons` | Listar cupons |
| `pb coupons activate <id>` | `PUT /coupons/{id}/activate` | Ativar cupom |
| `pb coupons deactivate <id>` | `PUT /coupons/{id}/deactivate` | Inativar cupom |
| `pb invoices get <id>` | `GET /invoices/{id}` | Consultar fatura |
| `pb invoices payments <id>` | `GET /invoices/{id}/payments` | Listar pagamentos |
| `pb invoices refund <id>` | `POST /payments/{id}/refund` | Criar estorno |
| `pb invoices list-refunds <id>` | `GET /payments/{id}/refunds` | Listar estornos |
| `pb invoices get-payment <id>` | `GET /payments/{id}` | Consultar pagamento |
| `pb invoices list-payments` | `GET /payments` | Listar pagamentos c/ filtros |
| `pb invoices list-seller-refunds` | `GET /refunds` | Listar estornos do vendedor |
| `pb invoices get-refund <id>` | `GET /refunds/{id}` | Consultar estorno |
| `pb preferences get` | `GET /notification-preferences` | Preferências de notificação |
| `pb preferences update <json>` | `PUT /notification-preferences` | Alterar preferências |
| `pb preferences encryption-key-get` | `GET /encryption-key` | Chave de criptografia |
| `pb preferences encryption-key-create <json>` | `PUT /encryption-key` | Criar chave criptografia |
| `pb retries get <id>` | `GET /retries/{id}` | Consultar retentativa |
| `pb retries update <id>` | `PUT /retries/{id}` | Alterar retentativa |
| `pb retries manual-retry <subs_id>` | `PUT /subscriptions/{id}/retry` | Retentativa manual |
| `pb clubpag settings` | `GET /clubpag/settings` | Configurações |
| `pb clubpag update-settings` | `PUT /clubpag/settings` | Alterar config |
| `pb clubpag purchase` | `POST /clubpag/purchase` | Identificar compra |
| `pb clubpag benefits` | `GET /clubpag/benefits` | Listar benefícios |
| `pb clubpag redeem` | `POST /clubpag/benefits/redeem` | Resgatar benefício |
| `pb clubpag cashback` | `GET /clubpag/cashback` | Detalhes cashback |
| `pb clubpag coupons` | `GET /clubpag/coupons` | Cupons ClubPag |
| `pb chargebacks get <id>` | `GET /chargebacks/{id}` | Consultar chargeback |
| `pb chargebacks dispute <id> --file` | `POST /chargebacks/{id}/disputes` | Enviar disputa |
| `pb chargebacks get-dispute <cbk> <disp>` | `GET /chargebacks/{cbk}/disputes/{disp}` | Consultar disputa |
| `pb notifications create` | `POST /notifications/preferences` | Criar preferência |
| `pb notifications get <id>` | `GET /notifications/preferences/{id}` | Consultar preferência |
| `pb notifications list` | `GET /notifications/preferences` | Listar preferências |
| `pb notifications update <id>` | `PATCH /notifications/preferences/{id}` | Alterar preferência |
| `pb notifications delete <id>` | `DELETE /notifications/preferences/{id}` | Excluir preferência |
| `pb transfers create` | `POST /transfers` | Criar transferência |
| `pb transfers get <id>` | `GET /transfers/{id}` | Consultar transferência |
| `pb transfers list` | `GET /transfers` | Listar transferências |
| `pb edi movements --type --date` | `GET /movement/v3.00/{tipo}/{data}` | Extrato EDI |
| `pb webhooks verify` | — (offline) | Verificar assinatura SHA-256 |
