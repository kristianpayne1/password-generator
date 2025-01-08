mod helpers;
mod charsets;

use wasm_bindgen::prelude::*;
use rand::{Rng, thread_rng};
use crate::helpers::{check_includes_lowercase, check_includes_number, check_includes_symbols, check_includes_uppercase};

struct Config {
    length: usize,
    include_number: bool,
    include_uppercase: bool,
    include_lowercase: bool,
    include_symbols: bool,
}

impl Config {
    fn new(length: usize, include_number: Option<bool>, include_uppercase: Option<bool>, include_lowercase: Option<bool>, include_symbols: Option<bool>) -> Config {
        let _include_number = include_number.unwrap_or(true);
        let _include_uppercase = include_uppercase.unwrap_or(true);
        let _include_lowercase = include_lowercase.unwrap_or(true);
        let _include_symbols = include_symbols.unwrap_or(true);
        Config {length, include_number: _include_number, include_uppercase: _include_uppercase, include_lowercase: _include_lowercase, include_symbols: _include_symbols }
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate(length: usize, include_number: Option<bool>, include_uppercase: Option<bool>, include_lowercase: Option<bool>, include_symbols: Option<bool>) -> JsValue {
    let config = Config::new ( length, include_number, include_uppercase, include_lowercase, include_symbols );
    let charset = create_charset(&config);
    let password = create_password(&config, &charset);
    JsValue::from_str(&password)
}

#[wasm_bindgen]
pub fn check_strength(password: String) -> JsValue {
    let mut strength = password.len();
    if check_includes_number(&password) {
        strength += 1;
    }
    if check_includes_uppercase(&password) {
        strength += 1;
    }
    if check_includes_lowercase(&password) {
        strength += 1;
    }
    if check_includes_symbols(&password) {
        strength += 1;
    }
    let value = (strength as f64 / 24.0) * 100.0;
    JsValue::from_f64(value.clamp(0.0, 100.0))
}

fn create_charset(config: &Config) -> String {
    let mut charset:String = String::new();

    if config.include_number {
        charset.push_str(charsets::NUMBERS);
    }
    if config.include_uppercase {
        charset.push_str(charsets::UPPERCASE_CHARSET);
    }
    if config.include_lowercase {
        charset.push_str(charsets::LOWERCASE_CHARSET);
    }
    if config.include_symbols {
        charset.push_str(charsets::SYMBOLS);
    }

    charset
}

fn create_password(config: &Config, binding: &String) -> String {
    let charset = binding.as_bytes();
    let mut rng = thread_rng();

    let mut password:String = (0..config.length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    if config.include_number && !helpers::check_includes_number(&password) {
        password = create_password(&config, &binding);
    }
    if config.include_uppercase && !helpers::check_includes_uppercase(&password) {
        password = create_password(&config, &binding);
    }
    if config.include_lowercase && !helpers::check_includes_lowercase(&password) {
        password = create_password(&config, &binding);
    }
    if config.include_symbols && !helpers::check_includes_symbols(&password) {
        password = create_password(&config, &binding);
    }

    password
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_length() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert_eq!(generated.len(), 8);
    }

    #[test]
    fn password_includes_number() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(helpers::check_includes_number(&generated));
    }

    #[test]
    fn not_password_includes_number() {
        let config = Config{length: 8, include_number: false, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(!helpers::check_includes_number(&generated));
    }

    #[test]
    fn password_includes_uppercase() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(helpers::check_includes_uppercase(&generated));
    }

    #[test]
    fn not_password_includes_uppercase() {
        let config = Config{length: 8, include_number: true, include_uppercase: false, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(!helpers::check_includes_uppercase(&generated));
    }

    #[test]
    fn password_includes_lowercase() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(helpers::check_includes_lowercase(&generated));
    }

    #[test]
    fn not_password_includes_lowercase() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: false, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(!helpers::check_includes_lowercase(&generated));
    }

    #[test]
    fn password_includes_symbols() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: true};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(helpers::check_includes_symbols(&generated));
    }

    #[test]
    fn not_password_includes_symbols() {
        let config = Config{length: 8, include_number: true, include_uppercase: true, include_lowercase: true, include_symbols: false};
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert!(!helpers::check_includes_symbols(&generated));
    }
}
