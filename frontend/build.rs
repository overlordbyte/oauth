//! Loyiha identifikatorlarini kompilyatsiya vaqtida wasm ichiga kiritadi.
//!
//! Frontend brauzerda ishlaydi — u yerda muhit o'zgaruvchilari yo'q, shuning
//! uchun qiymatlar build paytida o'qilib `env!(...)` orqali beriladi.
//! Manba backend bilan bir xil: workspace ildizidagi `.env.{APP_ENV}` fayl.

use std::fs;

fn main() {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let env_file = format!("../.env.{app_env}");

    println!("cargo:rerun-if-env-changed=APP_ENV");
    println!("cargo:rerun-if-changed={env_file}");
    println!("cargo:rerun-if-changed=../.env");

    // Rust kodida `env!("PROJECT_NAME")` / `env!("PROJECT_DOMAIN")` orqali olinadi.
    // Yangi qiymat kerak bo'lsa — shu ro'yxatga qo'shish kifoya.
    for key in ["PROJECT_NAME", "PROJECT_DOMAIN"] {
        println!("cargo:rerun-if-env-changed={key}");
        let value = resolve(key, &env_file);
        println!("cargo:rustc-env={key}={value}");
    }
}

/// Tartib: haqiqiy muhit o'zgaruvchisi → `.env.{APP_ENV}` → `.env`
fn resolve(key: &str, env_file: &str) -> String {
    std::env::var(key)
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| read_key(env_file, key))
        .or_else(|| read_key("../.env", key))
        .unwrap_or_else(|| panic!("{key} sozlanmagan — {env_file} ga `{key}=...` qo'shing"))
}

/// `.env` uslubidagi fayldan bitta kalitni o'qiydi.
///
/// Atigi bitta qiymat kerak bo'lgani uchun to'liq dotenv kutubxonasi
/// build-dependency sifatida qo'shilmadi.
fn read_key(path: &str, key: &str) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    contents.lines().find_map(|line| {
        let line = line.trim();
        if line.starts_with('#') {
            return None;
        }
        let (k, v) = line.split_once('=')?;
        if k.trim() != key {
            return None;
        }
        Some(v.trim().trim_matches(['"', '\'']).to_string())
    })
}
