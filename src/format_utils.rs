use num_traits::ToPrimitive;
use serde::Serialize;

pub fn round_to_one_decimal<T: ToPrimitive>(num: T) -> f64 {
    if let Some(n) = num.to_f64() {
        (n * 10.0).round() / 10.0
    } else {
        0.0
    }
}

const GB: u64 = 1 << 30;

pub fn percent<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap_or(0.0);
    let b_f64 = b.to_f64().unwrap_or(0.0);

    if b_f64 > 0.0 {
        round_to_one_decimal((a_f64 / b_f64) * 100.0)
    } else {
        0.0 // Avoid division by zero
    }
}

pub fn bytes_to_gb<T: ToPrimitive>(num: T) -> f64 {
    if let Some(n) = num.to_f64() {
        let gb = n / GB as f64;
        (gb * 10.0).round() / 10.0
    } else {
        // Handle the case where the conversion fails (e.g., for non-numeric types)
        0.0
    }
}

pub fn format_bytes<T: ToPrimitive>(bytes: T) -> String {

    let bytes_f64 = bytes.to_f64().unwrap_or(0.0);

    const GB: u64 = 1 << 30; // 1 GiB in bytes
    const MB: u64 = 1 << 20; // 1 MiB in bytes

    if bytes_f64 as u64 >= GB {
        format!("{:.1}GB", bytes_f64 as f64 / GB as f64)
    } else {
        format!("{:.1}MB", bytes_f64 as f64 / MB as f64)
    }
}

pub fn format_percent<T: ToPrimitive>(a: T) -> String {
    let a_f64 = a.to_f64().unwrap_or(0.0);
    return format!("{:.1}%",a_f64 );// Avoid division by zero
}

pub fn calculate_format_percent<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> String {
    let a_f64 = a.to_f64().unwrap_or(0.0);
    let b_f64 = b.to_f64().unwrap_or(0.0);
    if b_f64 > 0.0 {
        return format!("{:.1}%", (a_f64 / b_f64) * 100.0);
    } else {
        return format!("{:.1}%",0.0 );// Avoid division by zero
    };
}

pub fn clear_terminal(){
    print!("\x1B[2J\x1B[3J\x1B[H");
}

pub fn print_json<T: Serialize>(body: &T) {
    let json = serde_json::to_string_pretty(body).expect("Failed to convert to JSON");
    println!("{}", json);
}