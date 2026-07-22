use comfy_table::{Table, presets::UTF8_FULL, modifiers::UTF8_ROUND_CORNERS};
use serde_json::Value;

pub fn print_json(value: &Value) {
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

pub fn print_table(headers: &[&str], rows: Vec<Vec<String>>) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    table.set_header(headers.iter().map(|h| comfy_table::Cell::new(*h).fg(comfy_table::Color::Cyan)));

    for row in rows {
        table.add_row(row);
    }
    println!("{table}");
}

pub fn print_object_table(title: &str, value: &Value) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        comfy_table::Cell::new("Campo").fg(comfy_table::Color::Cyan),
        comfy_table::Cell::new("Valor").fg(comfy_table::Color::Cyan),
    ]);

    if let Some(obj) = value.as_object() {
        for (key, val) in obj {
            let display_val = match val {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                Value::Array(a) => format!("[{} items]", a.len()),
                Value::Object(o) => format!("{{{} fields}}", o.len()),
            };
            table.add_row(vec![key.clone(), display_val]);
        }
    }

    println!("\n{title}");
    println!("{table}");
}

pub fn print_error(msg: &str) {
    eprintln!("\x1b[31m✗ {msg}\x1b[0m");
}

pub fn print_success(msg: &str) {
    println!("\x1b[32m✓ {msg}\x1b[0m");
}

pub fn print_info(msg: &str) {
    println!("\x1b[36m→ {msg}\x1b[0m");
}
