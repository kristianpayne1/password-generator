use crate::charsets;

pub fn check_includes_number(password: &str) -> bool {
    password.chars().any(|c| c.is_numeric())
}

pub fn check_includes_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_uppercase())
}

pub fn check_includes_lowercase(password: &str) -> bool {
    password.chars().any(|c| c.is_lowercase())
}

pub fn check_includes_symbols(password: &str) -> bool {
    password.chars().any(|c| charsets::SYMBOLS.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_number() {
        assert!(check_includes_number("hello123"));
    }

    #[test]
    fn not_includes_number() {
        assert!(!check_includes_number("hello"));
    }

    #[test]
    fn includes_uppercase() {
        assert!(check_includes_uppercase("Hello"));
    }

    #[test]
    fn not_includes_uppercase() {
        assert!(!check_includes_uppercase("hello"));
    }

    #[test]
    fn includes_lowercase() {
        assert!(check_includes_lowercase("HELLo"));
    }

    #[test]
    fn not_includes_lowercase() {
        assert!(!check_includes_lowercase("HELLO"));
    }

    #[test]
    fn includes_symbols() {
        assert!(check_includes_symbols("hello!"));
    }

    #[test]
    fn not_includes_symbols() {
        assert!(!check_includes_symbols("5ldM5hXN"));
    }
}
