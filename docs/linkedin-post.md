🚀 Projeto Open Source: pb — CLI do PagBank em Rust

Passei as últimas semanas desenvolvendo uma ferramenta de linha de comando 100% em Rust que integra todas as APIs do PagBank em um único terminal.

🔧 **O que faz?**
- Pagamentos via PIX, cartão de crédito/débito e boleto
- Geração e exportação de QR Code PIX (base64 + PNG)
- Gestão de assinaturas recorrentes (planos, assinantes, cupons, faturas)
- Checkout PagBank, ClubPag (cashback), Connect (OAuth2)
- Divisão de pagamento entre múltiplos recebedores (split)
- Verificação de autenticidade de webhooks via SHA-256
- Autocomplete para bash, zsh, fish e powershell

📦 **Distribuição:**
- crates.io — cargo install pb
- AUR — yay -S pagbank-cli-bin
- GitHub Releases (.deb, .rpm, binários Linux/macOS/Windows)
- Homebrew (em breve)

🦀 **Stack:**
- Rust (tokio async, reqwest, clap, serde)
- SDK desacoplada do CLI — dois crates separados
- CI/CD com GitHub Actions (fmt, test, clippy, build, publish)
- 35+ testes unitários

💡 **Por que Rust?**
Performance nativa, zero runtime, segurança de memória, binário único sem dependências.

🔗 https://github.com/vinycalves/pagbank-cli

Feedbacks, issues e PRs são muito bem-vindos!

#Rust #OpenSource #PagBank #CLI #Fintech #Brasil #Dev
