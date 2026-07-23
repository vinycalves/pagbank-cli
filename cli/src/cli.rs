use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "pb",
    about = "CLI para as APIs do PagBank",
    version,
    long_about = None
)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub environment: Option<String>,

    #[arg(short, long, global = true, value_name = "FORMAT")]
    #[arg(default_value = "table")]
    pub output: OutputFormat,

    #[arg(long, global = true)]
    pub verbose: bool,

    #[arg(long, global = true)]
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

    #[command(about = "Gerenciar aplicações Connect")]
    Connect {
        #[command(subcommand)]
        action: ConnectAction,
    },

    #[command(about = "Gerenciar certificados digitais")]
    Certs {
        #[command(subcommand)]
        action: CertsAction,
    },

    #[command(about = "Gerenciar contas (Cadastro)")]
    Accounts {
        #[command(subcommand)]
        action: AccountsAction,
    },

    #[command(about = "Gerenciar pedidos e pagamentos")]
    Orders {
        #[command(subcommand)]
        action: OrdersAction,
    },

    #[command(about = "Gerenciar checkouts")]
    Checkouts {
        #[command(subcommand)]
        action: CheckoutsAction,
    },

    #[command(about = "Gerenciar planos de recorrência")]
    Plans {
        #[command(subcommand)]
        action: PlansAction,
    },

    #[command(about = "Gerenciar assinantes")]
    Subscribers {
        #[command(subcommand)]
        action: SubscribersAction,
    },

    #[command(about = "Gerenciar assinaturas")]
    Subscriptions {
        #[command(subcommand)]
        action: SubscriptionsAction,
    },

    #[command(about = "Gerenciar cupons")]
    Coupons {
        #[command(subcommand)]
        action: CouponsAction,
    },

    #[command(about = "Gerenciar faturas e pagamentos recorrentes")]
    Invoices {
        #[command(subcommand)]
        action: InvoicesAction,
    },

    #[command(about = "ClubPag")]
    Clubpag {
        #[command(subcommand)]
        action: ClubPagAction,
    },

    #[command(about = "Verificar autenticidade de webhooks")]
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
    #[command(about = "Configurar token de autenticação")]
    Login {
        #[arg(short, long)]
        token: String,
    },
    #[command(about = "Remover credenciais")]
    Logout,
    #[command(about = "Verificar autenticação atual")]
    Status,
}

#[derive(Subcommand)]
pub enum ConfigAction {
    #[command(about = "Setup interativo")]
    Init,
    #[command(about = "Definir valor de configuração")]
    Set { key: String, value: String },
    #[command(about = "Ver valor de configuração")]
    Get { key: String },
    #[command(about = "Mostrar configuração completa")]
    Show,
}

#[derive(Subcommand)]
pub enum KeysAction {
    #[command(about = "Criar chave pública")]
    Create,
    #[command(about = "Consultar chave pública")]
    Get { id: String },
    #[command(about = "Alterar chave pública")]
    Update { id: String },
}

#[derive(Subcommand)]
pub enum ConnectAction {
    #[command(about = "Criar aplicação Connect")]
    AppCreate {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        site: Option<String>,
        #[arg(long)]
        redirect_uri: Option<String>,
        #[arg(long)]
        logo: Option<String>,
    },
    #[command(about = "Consultar aplicação")]
    AppGet { id: String },
    #[command(about = "Gerar URL de autorização")]
    Authorize {
        #[arg(long)]
        app_id: String,
        #[arg(long)]
        redirect_uri: String,
        #[arg(long, default_value = "charge")]
        scope: String,
    },
    #[command(about = "Obter access token OAuth2")]
    Token {
        #[arg(long)]
        code: String,
    },
    #[command(about = "Renovar access token")]
    TokenRefresh {
        #[arg(long)]
        refresh_token: String,
    },
    #[command(about = "Revogar access token")]
    TokenRevoke {
        #[arg(long)]
        token: String,
    },
}

#[derive(Subcommand)]
pub enum CertsAction {
    #[command(about = "Criar certificado digital")]
    Create {
        #[arg(long)]
        certificate: String,
        #[arg(long)]
        password: String,
    },
}

#[derive(Subcommand)]
pub enum AccountsAction {
    #[command(about = "Criar conta")]
    Create {
        #[arg(long)]
        reference_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        tax_id: String,
        #[arg(long, default_value = "SELLER")]
        r#type: String,
        #[arg(long)]
        tos_ip: String,
    },
    #[command(about = "Consultar conta")]
    Get { id: String },
}

#[derive(Subcommand)]
pub enum OrdersAction {
    #[command(about = "Criar pedido")]
    Create {
        #[arg(long)]
        reference_id: Option<String>,
        #[arg(long)]
        customer_name: String,
        #[arg(long)]
        customer_email: String,
        #[arg(long)]
        customer_tax_id: String,
        #[arg(long)]
        item: String,
        #[arg(long, default_value = "1")]
        item_qty: i32,
        #[arg(long)]
        item_amount: i64,
        #[arg(long)]
        method: String,
        #[arg(long)]
        card_number: Option<String>,
        #[arg(long)]
        card_exp_month: Option<i32>,
        #[arg(long)]
        card_exp_year: Option<i32>,
        #[arg(long)]
        card_cvv: Option<String>,
        #[arg(long)]
        card_holder_name: Option<String>,
        #[arg(long)]
        card_holder_tax_id: Option<String>,
        #[arg(long)]
        installments: Option<i32>,
        #[arg(long)]
        notification_url: Option<String>,
        #[arg(long)]
        qr_amount: Option<i64>,
        #[arg(long)]
        pix: bool,
        #[arg(long, num_args = 0..=1)]
        pix_save: Option<String>,
    },
    #[command(about = "Consultar pedido")]
    Get { id: String },
    #[command(about = "Listar pedidos")]
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
    #[command(about = "Pagar pedido")]
    Pay {
        order_id: String,
        #[arg(long)]
        method: String,
        #[arg(long)]
        card_number: Option<String>,
        #[arg(long)]
        card_exp_month: Option<i32>,
        #[arg(long)]
        card_exp_year: Option<i32>,
        #[arg(long)]
        card_cvv: Option<String>,
        #[arg(long)]
        card_holder_name: Option<String>,
        #[arg(long)]
        card_holder_tax_id: Option<String>,
        #[arg(long)]
        installments: Option<i32>,
        #[arg(long)]
        card_id: Option<String>,
    },
    #[command(about = "Capturar pagamento autorizado")]
    Capture { charge_id: String },
    #[command(about = "Cancelar pagamento")]
    Cancel { charge_id: String },
    #[command(about = "Consultar divisão do pagamento")]
    Split { order_id: String },
    #[command(about = "Liberar divisão com custódia")]
    SplitRelease { order_id: String },
    #[command(about = "Consultar taxas de uma transação")]
    Fees { charge_id: String },
    #[command(about = "Validar e armazenar cartão")]
    CardStore {
        #[arg(long)]
        number: String,
        #[arg(long)]
        exp_month: i32,
        #[arg(long)]
        exp_year: i32,
        #[arg(long)]
        security_code: String,
        #[arg(long)]
        holder_name: String,
        #[arg(long)]
        holder_tax_id: String,
    },
}

#[derive(Subcommand)]
pub enum CheckoutsAction {
    #[command(about = "Criar checkout")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        amount: i64,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        redirect_url: Option<String>,
        #[arg(long)]
        payment_methods: Option<String>,
    },
    #[command(about = "Consultar checkout")]
    Get { id: String },
    #[command(about = "Ativar checkout")]
    Activate { id: String },
    #[command(about = "Inativar checkout")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum PlansAction {
    #[command(about = "Criar plano")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        amount: i64,
        #[arg(long)]
        period: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        reference_id: Option<String>,
        #[arg(long)]
        setup_fee: Option<i64>,
        #[arg(long)]
        billing_cycles: Option<i32>,
        #[arg(long)]
        trial_length: Option<i32>,
        #[arg(long)]
        trial_unit: Option<String>,
    },
    #[command(about = "Consultar plano")]
    Get { id: String },
    #[command(about = "Listar planos")]
    List {
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
    #[command(about = "Alterar plano")]
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        amount: Option<i64>,
        #[arg(long)]
        description: Option<String>,
    },
    #[command(about = "Ativar plano")]
    Activate { id: String },
    #[command(about = "Inativar plano")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum SubscribersAction {
    #[command(about = "Criar assinante")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        tax_id: String,
        #[arg(long)]
        reference_id: Option<String>,
        #[arg(long)]
        phone_area: Option<String>,
        #[arg(long)]
        phone_number: Option<String>,
        #[arg(long)]
        phone_type: Option<String>,
    },
    #[command(about = "Consultar assinante")]
    Get { id: String },
    #[command(about = "Listar assinantes")]
    List {
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
    #[command(about = "Alterar dados cadastrais")]
    UpdateProfile {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        tax_id: Option<String>,
    },
    #[command(about = "Alterar dados de pagamento")]
    UpdatePayment {
        id: String,
        #[arg(long)]
        card_number: String,
        #[arg(long)]
        card_exp_month: i32,
        #[arg(long)]
        card_exp_year: i32,
        #[arg(long)]
        card_cvv: String,
        #[arg(long)]
        card_holder_name: String,
        #[arg(long)]
        card_holder_tax_id: String,
    },
}

#[derive(Subcommand)]
pub enum SubscriptionsAction {
    #[command(about = "Criar assinatura")]
    Create {
        #[arg(long)]
        plan_id: String,
        #[arg(long)]
        subscriber_id: String,
        #[arg(long)]
        start_at: Option<String>,
    },
    #[command(about = "Consultar assinatura")]
    Get { id: String },
    #[command(about = "Listar assinaturas")]
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
    #[command(about = "Alterar assinatura")]
    Update {
        id: String,
        #[arg(long)]
        plan_id: Option<String>,
    },
    #[command(about = "Cancelar assinatura")]
    Cancel { id: String },
    #[command(about = "Suspender assinatura")]
    Suspend { id: String },
    #[command(about = "Ativar assinatura")]
    Activate { id: String },
    #[command(about = "Listar faturas da assinatura")]
    Invoices { id: String },
}

#[derive(Subcommand)]
pub enum CouponsAction {
    #[command(about = "Criar cupom")]
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        discount_type: String,
        #[arg(long)]
        discount_value: i64,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        reference_id: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
    },
    #[command(about = "Consultar cupom")]
    Get { id: String },
    #[command(about = "Listar cupons")]
    List {
        #[arg(long, default_value = "1")]
        page: i32,
        #[arg(long, default_value = "20")]
        per_page: i32,
    },
    #[command(about = "Ativar cupom")]
    Activate { id: String },
    #[command(about = "Inativar cupom")]
    Deactivate { id: String },
}

#[derive(Subcommand)]
pub enum InvoicesAction {
    #[command(about = "Consultar fatura")]
    Get { id: String },
    #[command(about = "Listar pagamentos de uma fatura")]
    Payments { invoice_id: String },
    #[command(about = "Criar estorno")]
    Refund {
        payment_id: String,
        #[arg(long)]
        amount: Option<i64>,
    },
    #[command(about = "Listar estornos")]
    ListRefunds { payment_id: String },
    #[command(about = "Consultar pagamento recorrente")]
    GetPayment { payment_id: String },
}

#[derive(Subcommand)]
pub enum ClubPagAction {
    #[command(about = "Consultar configurações")]
    Settings,
    #[command(about = "Atualizar configurações")]
    UpdateSettings {
        #[arg(long)]
        enabled: bool,
    },
    #[command(about = "Identificar compra")]
    Purchase {
        #[arg(long)]
        order_id: String,
        #[arg(long)]
        amount: i64,
    },
    #[command(about = "Listar benefícios")]
    Benefits,
    #[command(about = "Resgatar benefício")]
    Redeem {
        #[arg(long)]
        benefit_id: String,
        #[arg(long)]
        amount: Option<i64>,
    },
    #[command(about = "Ver detalhes de cashback")]
    Cashback,
    #[command(about = "Listar cupons ClubPag")]
    Coupons,
}

#[derive(Subcommand)]
pub enum WebhooksAction {
    #[command(about = "Verificar autenticidade de um webhook")]
    Verify {
        #[arg(long)]
        token: String,
        #[arg(long)]
        signature: String,
        payload_file: Option<String>,
    },
}
