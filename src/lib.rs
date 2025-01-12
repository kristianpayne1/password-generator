mod helpers;
mod charsets;

use wasm_bindgen::prelude::*;
use rand::{Rng, thread_rng};
use helpers::{check_includes_lowercase, check_includes_number, check_includes_symbols, check_includes_uppercase};

struct Config {
    length: usize,
    include_number: bool,
    include_uppercase: bool,
    include_lowercase: bool,
    include_symbols: bool,
}

impl Config {
    fn new(
        length: usize,
        include_number: Option<bool>,
        include_uppercase: Option<bool>,
        include_lowercase: Option<bool>,
        include_symbols: Option<bool>,
    ) -> Self {
        Self {
            length,
            include_number: include_number.unwrap_or(true),
            include_uppercase: include_uppercase.unwrap_or(true),
            include_lowercase: include_lowercase.unwrap_or(true),
            include_symbols: include_symbols.unwrap_or(true),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            length: 8,
            include_number: true,
            include_uppercase: true,
            include_lowercase: true,
            include_symbols: true,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate(
    length: usize,
    include_number: Option<bool>,
    include_uppercase: Option<bool>,
    include_lowercase: Option<bool>,
    include_symbols: Option<bool>,
) -> JsValue {
    let config = Config::new(length, include_number, include_uppercase, include_lowercase, include_symbols);
    let charset = create_charset(&config);
    let password = create_password(&config, &charset);
    JsValue::from_str(&password)
}

#[wasm_bindgen]
pub fn check_strength(password: String) -> JsValue {
    let mut strength = password.len();
    strength += if check_includes_number(&password) { 1 } else { 0 };
    strength += if check_includes_uppercase(&password) { 1 } else { 0 };
    strength += if check_includes_lowercase(&password) { 1 } else { 0 };
    strength += if check_includes_symbols(&password) { 1 } else { 0 };
    let value = (strength as f64 / 24.0) * 100.0;
    JsValue::from_f64(value.clamp(0.0, 100.0))
}

fn create_charset(config: &Config) -> String {
    let mut charset = String::new();
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

fn create_password(config: &Config, charset: &str) -> String {
    let charset_bytes = charset.as_bytes();
    let mut rng = thread_rng();

    loop {
        let password: String = (0..config.length)
            .map(|_| {
                let idx = rng.gen_range(0..charset_bytes.len());
                charset_bytes[idx] as char
            })
            .collect();

        if validate_password(&password, config) {
            return password;
        }
    }
}

fn validate_password(password: &str, config: &Config) -> bool {
    (!config.include_number || check_includes_number(password))
        && (!config.include_uppercase || check_includes_uppercase(password))
        && (!config.include_lowercase || check_includes_lowercase(password))
        && (!config.include_symbols || check_includes_symbols(password))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_config(
        length: usize,
        include_number: bool,
        include_uppercase: bool,
        include_lowercase: bool,
        include_symbols: bool,
    ) -> Config {
        Config {
            length,
            include_number,
            include_uppercase,
            include_lowercase,
            include_symbols,
        }
    }

    #[test]
    fn password_length() {
        let config = setup_config(8, true, true, true, true);
        let charset = create_charset(&config);
        let generated = create_password(&config, &charset);
        assert_eq!(generated.len(), 8);
    }

    #[test]
    fn password_constraints() {
        let configs = vec![
            setup_config(8, true, true, true, true),
            setup_config(8, false, true, true, true),
            setup_config(8, true, false, true, true),
            setup_config(8, true, true, false, true),
            setup_config(8, true, true, true, false),
        ];

        for config in configs {
            let charset = create_charset(&config);
            let generated = create_password(&config, &charset);

            if config.include_number {
                assert!(check_includes_number(&generated));
            } else {
                assert!(!check_includes_number(&generated));
            }

            if config.include_uppercase {
                assert!(check_includes_uppercase(&generated));
            } else {
                assert!(!check_includes_uppercase(&generated));
            }

            if config.include_lowercase {
                assert!(check_includes_lowercase(&generated));
            } else {
                assert!(!check_includes_lowercase(&generated));
            }

            if config.include_symbols {
                assert!(check_includes_symbols(&generated));
            } else {
                assert!(!check_includes_symbols(&generated));
            }
        }
    }
}

