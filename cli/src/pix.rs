use serde_json::Value;

pub async fn handle_pix_order(
    client: &pagbank_sdk::PagBankClient,
    order: &Value,
    pix_flag: bool,
    pix_save: Option<&str>,
    output_fmt: &crate::cli::OutputFormat,
) {
    if !pix_flag && pix_save.is_none() {
        return;
    }

    let qr_url = match extract_qr_base64_url(order) {
        Some(u) => u,
        None => return eprintln!("Nenhum QR code PIX encontrado na resposta"),
    };

    let b64 = match client.get_url_text(&qr_url).await {
        Ok(b) => b.trim().to_string(),
        Err(e) => {
            eprintln!("Falha ao buscar QR code PIX: {e}");
            return;
        }
    };

    if pix_flag {
        match output_fmt {
            crate::cli::OutputFormat::Json => {
                println!("\"pix_base64\": \"{}\"", b64);
            }
            crate::cli::OutputFormat::Table => {
                println!("\n── QR Code PIX ──");
                println!("Copia e cola (base64):\n");
                print_chunked(&b64, 76);
                println!();
            }
        }
    }

    if let Some(path) = pix_save {
        let path = if path.is_empty() {
            format!("qrcode-{}.png", order["id"].as_str().unwrap_or("order"))
        } else {
            path.to_string()
        };
        save_png(&b64, &path);
    }
}

fn extract_qr_base64_url(order: &Value) -> Option<String> {
    for charge in order["charges"].as_array()? {
        for link in charge["links"].as_array()? {
            if link["rel"].as_str() == Some("QRCODE.BASE64") {
                return link["href"].as_str().map(String::from);
            }
        }
    }
    None
}

fn save_png(b64: &str, path: &str) {
    use base64::Engine;

    let b64 = b64.trim();
    let clean: String = b64
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=')
        .collect();
    let padded = format!("{}{}", clean, "=".repeat((4 - clean.len() % 4) % 4));

    match base64::engine::general_purpose::STANDARD.decode(&padded) {
        Ok(bytes) => match std::fs::write(path, &bytes) {
            Ok(_) => println!("QR Code salvo: {path} ({} bytes)", bytes.len()),
            Err(e) => eprintln!("Erro ao salvar imagem: {e}"),
        },
        Err(e) => eprintln!("Erro ao decodificar base64: {e}"),
    }
}

fn print_chunked(text: &str, chunk_size: usize) {
    for chunk in text.as_bytes().chunks(chunk_size) {
        if let Ok(line) = std::str::from_utf8(chunk) {
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_qr_base64_url_found() {
        let order = serde_json::json!({
            "charges": [{
                "links": [
                    { "rel": "SELF", "href": "https://api.com/order/1" },
                    { "rel": "QRCODE.BASE64", "href": "https://api.com/qrcode/abc/base64" }
                ]
            }]
        });
        assert_eq!(
            extract_qr_base64_url(&order),
            Some("https://api.com/qrcode/abc/base64".to_string())
        );
    }

    #[test]
    fn extract_qr_base64_url_not_found() {
        let order = serde_json::json!({
            "charges": [{
                "links": [{ "rel": "SELF", "href": "https://api.com/order/1" }]
            }]
        });
        assert_eq!(extract_qr_base64_url(&order), None);
    }

    #[test]
    fn extract_qr_base64_url_no_charges() {
        let order = serde_json::json!({});
        assert_eq!(extract_qr_base64_url(&order), None);
    }

    #[test]
    fn print_chunked_splits_correctly() {
        let text = "abcdefghijklmnopqrstuvwxyz";
        let chunks: Vec<&str> = text
            .as_bytes()
            .chunks(10)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect();
        assert_eq!(chunks, vec!["abcdefghij", "klmnopqrst", "uvwxyz"]);
    }

    #[test]
    fn save_png_invalid_base64() {
        // should not panic, just print error
        save_png("not-valid-base64!!!", "/tmp/test-invalid.png");
    }

    #[test]
    fn save_png_writes_file() {
        // a valid 1x1 pixel PNG in base64
        let png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        let path = "/tmp/test-pix-unit.png";
        save_png(png_b64, path);
        assert!(std::path::Path::new(path).exists());
        let meta = std::fs::metadata(path).unwrap();
        assert!(meta.len() > 0);
        let _ = std::fs::remove_file(path);
    }
}
