# API PagBank — Mapeamento de Conceitos

Este documento mapeia como a API do PagBank funciona, as relações entre os
endpoints e como o CLI `pb` os implementa.

## Arquitetura Geral

A API PagBank segue o padrão RESTful, com respostas em JSON. Três domínios
principais coexistem:

```
PagBank API
├── Pagamento (pagamentos avulsos)
│   ├── Account        ← conta de recebedor (seller)
│   ├── Order + Charge ← pedido + cobrança
│   ├── Checkout       ← página de pagamento simplificada
│   ├── Public Key     ← criptografia de cartão
│   ├── Connect        ← OAuth2 (agir em nome de outro usuário)
│   └── Certificate    ← mTLS (segurança extra)
│
├── Recorrência (assinaturas)
│   ├── Plan           ← modelo de cobrança
│   ├── Subscriber     ← cliente da recorrência
│   ├── Subscription   ← vínculo plano + assinante
│   ├── Invoice        ← fatura gerada
│   ├── Coupon         ← desconto
│   └── Retry          ← retentativa de cobrança
│
└── ClubPag (fidelidade)
    └── ClubPag        ← cashback, benefícios, cupons
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

## Mapeamento CLI → API

| Comando CLI | Endpoint | Descrição |
|------------|----------|-----------|
| `pb keys create --type CARD` | `POST /public-keys` | Criar chave pública |
| `pb keys get <id>` | `GET /public-keys/{id}` | Consultar chave |
| `pb keys update <id>` | `PUT /public-keys/{id}` | Alterar chave |
| `pb connect app-create` | `POST /oauth2/application` | Criar app Connect |
| `pb connect app-get <id>` | `GET /oauth2/application/{id}` | Consultar app |
| `pb connect authorize` | `GET /oauth2/authorize` | Gerar URL de autorização |
| `pb connect token` | `POST /oauth2/token` | Obter access token |
| `pb connect token-refresh` | `POST /oauth2/token/refresh` | Renovar token |
| `pb connect token-revoke` | `POST /oauth2/token/revoke` | Revogar token |
| `pb certs create` | `POST /certificates` | Criar certificado mTLS |
| `pb accounts create` | `POST /accounts` | Criar conta (seller) |
| `pb accounts get <id>` | `GET /accounts/{id}` | Consultar conta |
| `pb orders create` | `POST /orders` | Criar pedido |
| `pb orders get <id>` | `GET /orders/{id}` | Consultar pedido |
| `pb orders list` | `GET /orders` | Listar pedidos |
| `pb orders pay <id>` | `POST /orders/{id}/pay` | Pagar pedido |
| `pb orders split <id>` | `GET /orders/{id}/splits` | Consultar split |
| `pb orders capture <id>` | `POST /charges/{id}/capture` | Capturar |
| `pb orders cancel <id>` | `POST /charges/{id}/cancel` | Cancelar |
| `pb orders fees <id>` | `GET /charges/{id}/costs` | Consultar taxas |
| `pb orders card-store` | `POST /cards` | Armazenar cartão |
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
| `pb clubpag settings` | `GET /clubpag/settings` | Configurações |
| `pb clubpag update-settings` | `PUT /clubpag/settings` | Alterar config |
| `pb clubpag purchase` | `POST /clubpag/purchase` | Identificar compra |
| `pb clubpag benefits` | `GET /clubpag/benefits` | Listar benefícios |
| `pb clubpag redeem` | `POST /clubpag/benefits/redeem` | Resgatar benefício |
| `pb clubpag cashback` | `GET /clubpag/cashback` | Detalhes cashback |
| `pb clubpag coupons` | `GET /clubpag/coupons` | Cupons ClubPag |
| `pb webhooks verify` | — (offline) | Verificar assinatura SHA-256 |
