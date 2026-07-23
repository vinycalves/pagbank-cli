use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "pb",
    about = "CLI para as APIs do PagBank",
    version,
    long_about = "CLI completa em Rust para consumir as APIs do PagBank. 
Documentação: https://github.com/vinycalves/pagbank-cli
Reporte bugs: https://github.com/vinycalves/pagbank-cli/issues"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        help = "Ambiente da API (sandbox ou production)"
    )]
    pub environment: Option<String>,

    #[arg(
        short,
        long,
        global = true,
        value_name = "FORMAT",
        default_value = "table",
        help = "Formato de saída (table ou json)"
    )]
    pub output: OutputFormat,

    #[arg(long, global = true, help = "Exibir informações de depuração")]
    pub verbose: bool,

    #[arg(
        long,
        global = true,
        help = "Caminho customizado para o arquivo de configuração"
    )]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Gerenciar autenticação")]
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    #[command(about = "Configurações do CLI")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    #[command(about = "Gerenciar chaves públicas")]
    Keys {
        #[command(subcommand)]
        action: KeysAction,
    },
    #[command(about = "Gerenciar aplicações Connect (OAuth2)")]
    Connect {
        #[command(subcommand)]
        action: ConnectAction,
    },
    #[command(about = "Gerenciar certificados digitais (mTLS)")]
    Certs {
        #[command(subcommand)]
        action: CertsAction,
    },
    #[command(about = "Gerenciar contas de recebedor (sellers)")]
    Accounts {
        #[command(subcommand)]
        action: AccountsAction,
    },
    #[command(about = "Gerenciar pedidos e pagamentos")]
    Orders {
        #[command(subcommand)]
        action: OrdersAction,
    },
    #[command(about = "Gerenciar checkouts PagBank")]
    Checkouts {
        #[command(subcommand)]
        action: CheckoutsAction,
    },
    #[command(about = "Gerenciar planos de recorrência")]
    Plans {
        #[command(subcommand)]
        action: PlansAction,
    },
    #[command(about = "Gerenciar assinantes (clientes da recorrência)")]
    Subscribers {
        #[command(subcommand)]
        action: SubscribersAction,
    },
    #[command(about = "Gerenciar assinaturas (vínculo plano + assinante)")]
    Subscriptions {
        #[command(subcommand)]
        action: SubscriptionsAction,
    },
    #[command(about = "Gerenciar cupons de desconto")]
    Coupons {
        #[command(subcommand)]
        action: CouponsAction,
    },
    #[command(about = "Gerenciar faturas e estornos")]
    Invoices {
        #[command(subcommand)]
        action: InvoicesAction,
    },
    #[command(about = "Gerenciar ClubPag (benefícios, cashback, cupons)")]
    Clubpag {
        #[command(subcommand)]
        action: ClubPagAction,
    },
    #[command(about = "Verificar autenticidade de webhooks via SHA-256")]
    Webhooks {
        #[command(subcommand)]
        action: WebhooksAction,
    },
    #[command(about = "Gerar script de autocomplete para o shell")]
    Completion { shell: Shell },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    #[clap(name = "powershell")]
    Powershell,
    Elvish,
}

#[derive(Subcommand)]
pub enum AuthAction {
    #[command(about = "Salvar token de autenticação no config")]
    Login {
        #[arg(
            short,
            long,
            help = "Token de autenticação Bearer do PagBank (obter no painel PagBank)"
        )]
        token: String,
    },
    #[command(about = "Remover token de autenticação do config")]
    Logout,
    #[command(about = "Exibir status da autenticação atual")]
    Status,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    #[command(about = "Iniciar configuração interativa (assistente passo a passo)")]
    Init,
    #[command(about = "Definir um valor de configuração")]
    Set { key: String, value: String },
    #[command(about = "Obter o valor de uma chave de configuração")]
    Get { key: String },
    #[command(about = "Exibir configuração completa")]
    Show,
}

#[derive(Subcommand)]
pub enum KeysAction {
    #[command(about = "Criar nova chave pública para criptografia de cartão")]
    Create {
        #[arg(
            long,
            help = "Tipo da chave: CARD, ANDROID, IOS, WEB (define onde o cartão criptografado será usado)"
        )]
        r#type: String,
    },
    #[command(about = "Consultar chave pública pelo ID")]
    Get { id: String },
    #[command(about = "Alterar chave pública existente")]
    Update { id: String },
}

#[derive(Subcommand)]
pub enum ConnectAction {
    #[command(about = "Criar nova aplicação OAuth2 Connect")]
    AppCreate {
        #[arg(short, long, help = "Nome da aplicação (ex: 'Minha Loja Online')")]
        name: String,
        #[arg(long, help = "Descrição da aplicação")]
        description: Option<String>,
        #[arg(long, help = "URL do site da aplicação")]
        site: Option<String>,
        #[arg(
            long,
            help = "URL de redirecionamento OAuth (para onde o usuário volta após autorizar)"
        )]
        redirect_uri: Option<String>,
        #[arg(long, help = "URL do logo da aplicação")]
        logo: Option<String>,
    },
    #[command(about = "Consultar dados de uma aplicação Connect")]
    AppGet { id: String },
    #[command(about = "Gerar URL de autorização OAuth2 para redirecionar o usuário")]
    Authorize {
        #[arg(long, help = "ID da aplicação Connect")]
        app_id: String,
        #[arg(
            long,
            help = "URL de redirecionamento (deve bater com o cadastrado na app)"
        )]
        redirect_uri: String,
        #[arg(
            long,
            default_value = "charge",
            help = "Escopo de permissões (ex: charge, payments)"
        )]
        scope: String,
    },
    #[command(about = "Trocar código de autorização por access token")]
    Token {
        #[arg(long, help = "Código de autorização recebido após o redirect OAuth")]
        code: String,
    },
    #[command(about = "Renovar access token expirado")]
    TokenRefresh {
        #[arg(long, help = "Refresh token recebido junto com o access token")]
        refresh_token: String,
    },
    #[command(about = "Revogar access token (invalida o token imediatamente)")]
    TokenRevoke {
        #[arg(long, help = "Access token a ser revogado")]
        token: String,
    },
}

#[derive(Subcommand)]
pub enum CertsAction {
    #[command(about = "Criar certificado digital para autenticação mTLS")]
    Create {
        #[arg(long, help = "Conteúdo ou caminho do certificado .p12")]
        certificate: String,
        #[arg(long, help = "Senha do certificado")]
        password: String,
    },
}

#[derive(Subcommand)]
pub enum AccountsAction {
    #[command(about = "Criar conta de recebedor (seller) para split de pagamentos")]
    Create {
        #[arg(
            long,
            help = "Identificador único da conta no seu sistema (ex: 'vend-001')"
        )]
        reference_id: String,
        #[arg(long, help = "Nome completo ou razão social do recebedor")]
        name: String,
        #[arg(long, help = "E-mail do recebedor")]
        email: String,
        #[arg(long, help = "CPF ou CNPJ do recebedor (apenas números)")]
        tax_id: String,
        #[arg(
            long,
            default_value = "SELLER",
            help = "Tipo da conta (SELLER, MERCHANT, etc.)"
        )]
        r#type: String,
        #[arg(long, help = "IP da requisição de aceite dos termos de uso")]
        tos_ip: String,
    },
    #[command(about = "Consultar dados de uma conta pelo ID")]
    Get { id: String },
}

#[derive(Subcommand)]
pub enum OrdersAction {
    #[command(about = "Criar um novo pedido com cobrança")]
    Create {
        #[arg(
            long,
            help = "Identificador do pedido no seu sistema (ex: 'ped-2024-001')"
        )]
        reference_id: Option<String>,
        #[arg(long, help = "Nome completo do cliente")]
        customer_name: String,
        #[arg(long, help = "E-mail do cliente")]
        customer_email: String,
        #[arg(long, help = "CPF ou CNPJ do cliente (apenas números)")]
        customer_tax_id: String,
        #[arg(long, help = "Nome do produto/serviço (ex: 'Assinatura Premium')")]
        item: String,
        #[arg(long, default_value = "1", help = "Quantidade de itens")]
        item_qty: i32,
        #[arg(
            long,
            help = "Valor unitário do item em centavos (ex: 1000 = R$ 10,00)"
        )]
        item_amount: i64,
        #[arg(long, help = "Método de pagamento: pix, credit_card, debit_card")]
        method: String,
        #[arg(
            long,
            help = "Número do cartão de crédito/débito (completo, sem espaços)"
        )]
        card_number: Option<String>,
        #[arg(long, help = "Mês de validade do cartão (1-12)")]
        card_exp_month: Option<i32>,
        #[arg(long, help = "Ano de validade do cartão (ex: 2027)")]
        card_exp_year: Option<i32>,
        #[arg(long, help = "Código de segurança do cartão (CVV)")]
        card_cvv: Option<String>,
        #[arg(long, help = "Nome do titular do cartão (como está gravado no cartão)")]
        card_holder_name: Option<String>,
        #[arg(long, help = "CPF/CNPJ do titular do cartão")]
        card_holder_tax_id: Option<String>,
        #[arg(long, help = "Número de parcelas (apenas crédito)")]
        installments: Option<i32>,
        #[arg(
            long,
            help = "URL de notificação (webhook) para receber atualizações do pedido"
        )]
        notification_url: Option<String>,
        #[arg(
            long,
            help = "Valor do QR Code PIX em centavos (necessário apenas se diferente do total)"
        )]
        qr_amount: Option<i64>,
        #[arg(long, help = "Exibir QR Code PIX no terminal (base64)")]
        pix: bool,
        #[arg(long, num_args = 0..=1, help = "Salvar QR Code PIX como imagem PNG (padrão: qrcode-{id}.png)")]
        pix_save: Option<String>,
    },
    #[command(about = "Consultar pedido pelo ID PagBank")]
    Get { id: String },
    #[command(about = "Listar pedidos com filtros")]
    List {
        #[arg(long, help = "Filtrar por status (PAID, WAITING, CANCELED, etc.)")]
        status: Option<String>,
        #[arg(long, default_value = "1", help = "Número da página para paginação")]
        page: i32,
        #[arg(long, default_value = "20", help = "Itens por página (max 100)")]
        per_page: i32,
    },
    #[command(about = "Pagar um pedido (após criado)")]
    Pay {
        order_id: String,
        #[arg(long, help = "Método de pagamento: pix, credit_card, debit_card")]
        method: String,
        #[arg(long, help = "Número do cartão")]
        card_number: Option<String>,
        #[arg(long, help = "Mês de validade do cartão")]
        card_exp_month: Option<i32>,
        #[arg(long, help = "Ano de validade do cartão")]
        card_exp_year: Option<i32>,
        #[arg(long, help = "Código de segurança do cartão (CVV)")]
        card_cvv: Option<String>,
        #[arg(long, help = "Nome do titular do cartão")]
        card_holder_name: Option<String>,
        #[arg(long, help = "CPF/CNPJ do titular do cartão")]
        card_holder_tax_id: Option<String>,
        #[arg(long, help = "Número de parcelas (apenas crédito)")]
        installments: Option<i32>,
        #[arg(
            long,
            help = "ID do cartão armazenado anteriormente (alternativa ao número)"
        )]
        card_id: Option<String>,
    },
    #[command(about = "Capturar pagamento autorizado")]
    Capture { charge_id: String },
    #[command(about = "Cancelar pagamento")]
    Cancel { charge_id: String },
    #[command(about = "Consultar divisão do pagamento (splits)")]
    Split { order_id: String },
    #[command(about = "Liberar divisão com custódia")]
    SplitRelease { order_id: String },
    #[command(about = "Consultar taxas de uma transação")]
    Fees { charge_id: String },
    #[command(about = "Validar e armazenar cartão para uso futuro")]
    CardStore {
        #[arg(long, help = "Número do cartão")]
        number: String,
        #[arg(long, help = "Mês de validade")]
        exp_month: i32,
        #[arg(long, help = "Ano de validade")]
        exp_year: i32,
        #[arg(long, help = "Código de segurança (CVV)")]
        security_code: String,
        #[arg(long, help = "Nome do titular do cartão")]
        holder_name: String,
        #[arg(long, help = "CPF/CNPJ do titular do cartão")]
        holder_tax_id: String,
    },
}

#[derive(Subcommand)]
pub enum CheckoutsAction {
    #[command(about = "Criar um checkout personalizado")]
    Create {
        #[arg(long, help = "Nome do checkout (ex: 'Checkout Loja Virtual')")]
        name: String,
        #[arg(long, help = "Valor total em centavos (ex: 1990 = R$ 19,90)")]
        amount: i64,
        #[arg(long, help = "Descrição do checkout")]
        description: Option<String>,
        #[arg(long, help = "URL para redirecionar o cliente após o pagamento")]
        redirect_url: Option<String>,
        #[arg(
            long,
            help = "Métodos de pagamento aceitos (ex: 'CREDIT_CARD,BOLETO,PIX')"
        )]
        payment_methods: Option<String>,
    },
    #[command(about = "Consultar checkout pelo ID")]
    Get { id: String },
    #[command(about = "Ativar checkout")]
    Activate { id: String },
    #[command(about = "Inativar checkout")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum PlansAction {
    #[command(about = "Criar plano de assinatura recorrente")]
    Create {
        #[arg(long, help = "Nome do plano (ex: 'Premium Mensal')")]
        name: String,
        #[arg(long, help = "Valor em centavos (ex: 4990 = R$ 49,90)")]
        amount: i64,
        #[arg(long, help = "Período de cobrança: month, year, week")]
        period: String,
        #[arg(long, help = "Descrição detalhada do plano")]
        description: Option<String>,
        #[arg(long, help = "Identificador do plano no seu sistema")]
        reference_id: Option<String>,
        #[arg(long, help = "Taxa de adesão única em centavos")]
        setup_fee: Option<i64>,
        #[arg(
            long,
            help = "Número máximo de ciclos de cobrança (deixe vazio para indefinido)"
        )]
        billing_cycles: Option<i32>,
        #[arg(long, help = "Duração do período de teste gratuito")]
        trial_length: Option<i32>,
        #[arg(long, help = "Unidade do período de teste: day, month")]
        trial_unit: Option<String>,
    },
    #[command(about = "Consultar plano pelo ID")]
    Get { id: String },
    #[command(about = "Listar planos cadastrados")]
    List {
        #[arg(long, default_value = "1", help = "Número da página")]
        page: i32,
        #[arg(long, default_value = "20", help = "Itens por página")]
        per_page: i32,
    },
    #[command(about = "Alterar dados de um plano")]
    Update {
        id: String,
        #[arg(long, help = "Novo nome do plano")]
        name: Option<String>,
        #[arg(long, help = "Novo valor em centavos")]
        amount: Option<i64>,
        #[arg(long, help = "Nova descrição")]
        description: Option<String>,
    },
    #[command(about = "Ativar plano")]
    Activate { id: String },
    #[command(about = "Inativar plano")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum SubscribersAction {
    #[command(about = "Criar assinante (cliente da recorrência)")]
    Create {
        #[arg(long, help = "Nome completo do assinante")]
        name: String,
        #[arg(long, help = "E-mail do assinante")]
        email: String,
        #[arg(long, help = "CPF ou CNPJ do assinante (apenas números)")]
        tax_id: String,
        #[arg(long, help = "Identificador do assinante no seu sistema")]
        reference_id: Option<String>,
        #[arg(long, help = "DDD do telefone (ex: 11)")]
        phone_area: Option<String>,
        #[arg(long, help = "Número do telefone")]
        phone_number: Option<String>,
        #[arg(long, help = "Tipo do telefone (MOBILE, HOME, WORK)")]
        phone_type: Option<String>,
    },
    #[command(about = "Consultar assinante pelo ID")]
    Get { id: String },
    #[command(about = "Listar assinantes")]
    List {
        #[arg(long, default_value = "1", help = "Número da página")]
        page: i32,
        #[arg(long, default_value = "20", help = "Itens por página")]
        per_page: i32,
    },
    #[command(about = "Alterar dados cadastrais do assinante")]
    UpdateProfile {
        id: String,
        #[arg(long, help = "Novo nome")]
        name: Option<String>,
        #[arg(long, help = "Novo e-mail")]
        email: Option<String>,
        #[arg(long, help = "Novo CPF/CNPJ")]
        tax_id: Option<String>,
    },
    #[command(about = "Alterar dados de pagamento do assinante")]
    UpdatePayment {
        id: String,
        #[arg(long, help = "Número do novo cartão")]
        card_number: String,
        #[arg(long, help = "Mês de validade do novo cartão")]
        card_exp_month: i32,
        #[arg(long, help = "Ano de validade do novo cartão")]
        card_exp_year: i32,
        #[arg(long, help = "CVV do novo cartão")]
        card_cvv: String,
        #[arg(long, help = "Nome do titular do novo cartão")]
        card_holder_name: String,
        #[arg(long, help = "CPF/CNPJ do titular do novo cartão")]
        card_holder_tax_id: String,
    },
}

#[derive(Subcommand)]
pub enum SubscriptionsAction {
    #[command(about = "Criar assinatura (vincular plano a um assinante)")]
    Create {
        #[arg(long, help = "ID do plano contratado")]
        plan_id: String,
        #[arg(long, help = "ID do assinante")]
        subscriber_id: String,
        #[arg(
            long,
            help = "Data de início da assinatura (ISO 8601, ex: 2026-08-01T00:00:00-03:00)"
        )]
        start_at: Option<String>,
    },
    #[command(about = "Consultar assinatura pelo ID")]
    Get { id: String },
    #[command(about = "Listar assinaturas")]
    List {
        #[arg(long, help = "Filtrar por status (ACTIVE, CANCELED, SUSPENDED, etc.)")]
        status: Option<String>,
        #[arg(long, default_value = "1", help = "Número da página")]
        page: i32,
        #[arg(long, default_value = "20", help = "Itens por página")]
        per_page: i32,
    },
    #[command(about = "Alterar assinatura (ex: trocar de plano)")]
    Update {
        id: String,
        #[arg(long, help = "ID do novo plano")]
        plan_id: Option<String>,
    },
    #[command(about = "Cancelar assinatura")]
    Cancel { id: String },
    #[command(about = "Suspender assinatura temporariamente")]
    Suspend { id: String },
    #[command(about = "Reativar assinatura suspensa")]
    Activate { id: String },
    #[command(about = "Listar faturas geradas pela assinatura")]
    Invoices { id: String },
}

#[derive(Subcommand)]
pub enum CouponsAction {
    #[command(about = "Criar cupom de desconto")]
    Create {
        #[arg(long, help = "Nome do cupom (ex: 'DESCONTO10')")]
        name: String,
        #[arg(long, help = "Tipo do desconto: PERCENTAGE ou FIXED")]
        discount_type: String,
        #[arg(long, help = "Valor do desconto (percentual ou centavos)")]
        discount_value: i64,
        #[arg(long, help = "Descrição do cupom")]
        description: Option<String>,
        #[arg(long, help = "Identificador do cupom no seu sistema")]
        reference_id: Option<String>,
        #[arg(long, help = "Limite de usos do cupom (deixe vazio para ilimitado)")]
        limit: Option<i64>,
    },
    #[command(about = "Consultar cupom pelo ID")]
    Get { id: String },
    #[command(about = "Listar cupons cadastrados")]
    List {
        #[arg(long, default_value = "1", help = "Número da página")]
        page: i32,
        #[arg(long, default_value = "20", help = "Itens por página")]
        per_page: i32,
    },
    #[command(about = "Ativar cupom")]
    Activate { id: String },
    #[command(about = "Inativar cupom")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum InvoicesAction {
    #[command(about = "Consultar fatura pelo ID")]
    Get { id: String },
    #[command(about = "Listar pagamentos de uma fatura")]
    Payments { invoice_id: String },
    #[command(about = "Criar estorno de um pagamento")]
    Refund {
        payment_id: String,
        #[arg(long, help = "Valor do estorno em centavos (padrão: valor total)")]
        amount: Option<i64>,
    },
    #[command(about = "Listar estornos de um pagamento")]
    ListRefunds { payment_id: String },
    #[command(about = "Consultar pagamento recorrente pelo ID")]
    GetPayment { payment_id: String },
}

#[derive(Subcommand)]
pub enum ClubPagAction {
    #[command(about = "Consultar configurações do ClubPag")]
    Settings,
    #[command(about = "Atualizar configurações do ClubPag")]
    UpdateSettings {
        #[arg(long, help = "Ativar ou desativar o ClubPag")]
        enabled: bool,
    },
    #[command(about = "Identificar uma compra no ClubPag")]
    Purchase {
        #[arg(long, help = "ID do pedido associado")]
        order_id: String,
        #[arg(long, help = "Valor da compra em centavos")]
        amount: i64,
    },
    #[command(about = "Listar benefícios disponíveis")]
    Benefits,
    #[command(about = "Resgatar um benefício")]
    Redeem {
        #[arg(long, help = "ID do benefício a ser resgatado")]
        benefit_id: String,
        #[arg(long, help = "Valor a resgatar em centavos (se aplicável)")]
        amount: Option<i64>,
    },
    #[command(about = "Ver detalhes do cashback")]
    Cashback,
    #[command(about = "Listar cupons ClubPag")]
    Coupons,
}

#[derive(Subcommand)]
pub enum WebhooksAction {
    #[command(about = "Verificar assinatura SHA-256 de um webhook recebido")]
    Verify {
        #[arg(long, help = "Token do webhook (configurado no painel PagBank)")]
        token: String,
        #[arg(
            long,
            help = "Assinatura SHA-256 recebida no header x-authenticity-token"
        )]
        signature: String,
        #[arg(help = "Arquivo contendo o payload bruto do webhook (ou ler da stdin)")]
        payload_file: Option<String>,
    },
}
