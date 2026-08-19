pub fn validate_email(email: &str) -> Result<(), String> {
    let email = email.trim();
    if email.is_empty() {
        return Err("e-mail não informado".to_string());
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(format!("e-mail inválido: {email}"));
    }
    let domain = parts[1];
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return Err(format!("e-mail inválido: {email}"));
    }
    if email.contains(' ') {
        return Err(format!("e-mail inválido: {email}"));
    }
    Ok(())
}

pub fn validate_tax_id(tax_id: &str) -> Result<(), String> {
    let digits: String = tax_id.chars().filter(|c| c.is_ascii_digit()).collect();
    match digits.len() {
        11 => validate_cpf(&digits),
        14 => validate_cnpj(&digits),
        n => Err(format!(
            "CPF/CNPJ inválido: deve conter 11 (CPF) ou 14 (CNPJ) dígitos, encontrados {n}"
        )),
    }
}

fn validate_cpf(digits: &str) -> Result<(), String> {
    let all_same = digits.chars().all(|c| c == digits.chars().next().unwrap());
    if all_same {
        return Err("CPF inválido: dígitos repetidos".to_string());
    }

    let sum1: u32 = digits[..9]
        .chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap() * (10 - i as u32))
        .sum();
    let d1 = cpf_check_digit(sum1);
    if d1 != digits.chars().nth(9).unwrap().to_digit(10).unwrap() {
        return Err("CPF inválido: dígito verificador inválido".to_string());
    }

    let sum2: u32 = digits[..10]
        .chars()
        .enumerate()
        .map(|(i, c)| c.to_digit(10).unwrap() * (11 - i as u32))
        .sum();
    let d2 = cpf_check_digit(sum2);
    if d2 != digits.chars().nth(10).unwrap().to_digit(10).unwrap() {
        return Err("CPF inválido: dígito verificador inválido".to_string());
    }
    Ok(())
}

fn cpf_check_digit(sum: u32) -> u32 {
    let rest = (sum * 10) % 11;
    if rest == 10 {
        0
    } else {
        rest
    }
}

fn validate_cnpj(digits: &str) -> Result<(), String> {
    let all_same = digits.chars().all(|c| c == digits.chars().next().unwrap());
    if all_same {
        return Err("CNPJ inválido: dígitos repetidos".to_string());
    }

    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum1: u32 = digits[..12]
        .chars()
        .zip(weights1.iter())
        .map(|(c, w)| c.to_digit(10).unwrap() * w)
        .sum();
    let d1 = cnpj_check_digit(sum1);
    if d1 != digits.chars().nth(12).unwrap().to_digit(10).unwrap() {
        return Err("CNPJ inválido: dígito verificador inválido".to_string());
    }

    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum2: u32 = digits[..13]
        .chars()
        .zip(weights2.iter())
        .map(|(c, w)| c.to_digit(10).unwrap() * w)
        .sum();
    let d2 = cnpj_check_digit(sum2);
    if d2 != digits.chars().nth(13).unwrap().to_digit(10).unwrap() {
        return Err("CNPJ inválido: dígito verificador inválido".to_string());
    }
    Ok(())
}

fn cnpj_check_digit(sum: u32) -> u32 {
    let rest = sum % 11;
    if rest < 2 {
        0
    } else {
        11 - rest
    }
}

pub fn is_valid_card_number(number: &str) -> bool {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 13 || digits.len() > 19 {
        return false;
    }
    let mut sum = 0u32;
    let mut double = false;
    for c in digits.chars().rev() {
        let mut d = c.to_digit(10).unwrap();
        if double {
            d *= 2;
            if d > 9 {
                d -= 9;
            }
        }
        sum += d;
        double = !double;
    }
    sum.is_multiple_of(10)
}

pub struct CardInfo<'a> {
    pub number: &'a str,
    pub exp_month: Option<i32>,
    pub exp_year: Option<i32>,
    pub cvv: Option<&'a str>,
}

pub fn validate_card(card: &CardInfo) -> Result<(), String> {
    use chrono::Datelike;

    if !is_valid_card_number(card.number) {
        return Err("número de cartão inválido (falhou no algoritmo de Luhn)".to_string());
    }

    if let Some(m) = card.exp_month {
        if !(1..=12).contains(&m) {
            return Err(format!("mês de validade inválido: {m} (use 1-12)"));
        }
    }

    let current_year = chrono::Utc::now().year();
    if let Some(y) = card.exp_year {
        if !(current_year..=current_year + 30).contains(&y) {
            return Err(format!("ano de validade inválido: {y}"));
        }
    }

    if let Some(code) = card.cvv {
        let code = code.trim();
        if code.len() < 3 || code.len() > 4 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err("código de segurança (CVV) inválido: use 3 ou 4 dígitos".to_string());
        }
    }

    Ok(())
}

pub fn validate_amount(value: i64, field: &str) -> Result<(), String> {
    if value <= 0 {
        return Err(format!("{field} deve ser maior que zero (em centavos)"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valid() {
        assert!(validate_email("joao@example.com").is_ok());
        assert!(validate_email("joao.silva@sub.example.com.br").is_ok());
    }

    #[test]
    fn email_invalid() {
        assert!(validate_email("").is_err());
        assert!(validate_email("sem-arroba").is_err());
        assert!(validate_email("a@b").is_err());
        assert!(validate_email("com espaco@x.com").is_err());
        assert!(validate_email("@dominio.com").is_err());
        assert!(validate_email("nome@.dominio").is_err());
    }

    #[test]
    fn cpf_valid() {
        // CPF válido: 529.982.247-25
        assert!(validate_tax_id("52998224725").is_ok());
        assert!(validate_tax_id("529.982.247-25").is_ok());
    }

    #[test]
    fn cpf_invalid_check_digit() {
        assert!(validate_tax_id("52998224726").is_err());
    }

    #[test]
    fn cpf_repeated_digits() {
        assert!(validate_tax_id("11111111111").is_err());
    }

    #[test]
    fn cnpj_valid() {
        // CNPJ válido: 11.222.333/0001-81
        assert!(validate_tax_id("11222333000181").is_ok());
        assert!(validate_tax_id("11.222.333/0001-81").is_ok());
    }

    #[test]
    fn cnpj_invalid_check_digit() {
        assert!(validate_tax_id("11222333000182").is_err());
    }

    #[test]
    fn tax_id_wrong_length() {
        assert!(validate_tax_id("123").is_err());
        assert!(validate_tax_id("123456789012345").is_err());
    }

    #[test]
    fn luhn_valid_cards() {
        // 4242 4242 4242 4242 (Visa de teste) e 4000 0000 0000 0002
        assert!(is_valid_card_number("4242424242424242"));
        assert!(is_valid_card_number("4000000000000002"));
        assert!(is_valid_card_number("4242 4242 4242 4242"));
    }

    #[test]
    fn luhn_invalid_cards() {
        assert!(!is_valid_card_number("4242424242424241"));
        assert!(!is_valid_card_number("1234567890123456"));
        assert!(!is_valid_card_number("123")); // muito curto
        assert!(!is_valid_card_number("4242424242424242424242")); // longo demais
        assert!(!is_valid_card_number("")); // vazio
    }

    fn card<'a>(
        number: &'a str,
        month: Option<i32>,
        year: Option<i32>,
        cvv: Option<&'a str>,
    ) -> CardInfo<'a> {
        CardInfo {
            number,
            exp_month: month,
            exp_year: year,
            cvv,
        }
    }

    #[test]
    fn validate_card_ok() {
        assert!(
            validate_card(&card("4242424242424242", Some(12), Some(2027), Some("123"))).is_ok()
        );
    }

    #[test]
    fn validate_card_bad_luhn() {
        assert!(
            validate_card(&card("4242424242424241", Some(12), Some(2027), Some("123"))).is_err()
        );
    }

    #[test]
    fn validate_card_bad_month() {
        assert!(
            validate_card(&card("4242424242424242", Some(13), Some(2027), Some("123"))).is_err()
        );
        assert!(
            validate_card(&card("4242424242424242", Some(0), Some(2027), Some("123"))).is_err()
        );
    }

    #[test]
    fn validate_card_bad_year() {
        assert!(
            validate_card(&card("4242424242424242", Some(12), Some(1990), Some("123"))).is_err()
        );
    }

    #[test]
    fn validate_card_bad_cvv() {
        assert!(
            validate_card(&card("4242424242424242", Some(12), Some(2027), Some("12"))).is_err()
        );
        assert!(
            validate_card(&card("4242424242424242", Some(12), Some(2027), Some("12a"))).is_err()
        );
    }

    #[test]
    fn validate_amount_ok_and_err() {
        assert!(validate_amount(100, "valor").is_ok());
        assert!(validate_amount(0, "valor").is_err());
        assert!(validate_amount(-50, "valor").is_err());
    }
}
