
pub fn check_includes_number(password: &str) -> bool {
    let mut include_number = false;
    for char in password.chars() {
        if char.is_numeric() {
            include_number = true;
            break;
        }
    }
    include_number
}

pub fn check_includes_uppercase(password: &str) -> bool {
    let mut include_uppercase = false;
    for char in password.chars() {
        if char.is_uppercase() {
            include_uppercase = true;
            break;
        }
    }
    include_uppercase
}

pub fn check_includes_lowercase(password: &str) -> bool {
    let mut include_lowercase = false;
    for char in password.chars() {
        if char.is_lowercase() {
            include_lowercase = true;
            break;
        }
    }
    include_lowercase
}

pub fn check_includes_symbols(password: &str) -> bool {
    let mut includes_symbols = false;
    for char in password.chars() {
        if !char.is_alphabetic() {
            includes_symbols = true;
            break;
        }
    }
    includes_symbols
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
        assert!(!check_includes_symbols("Hello"));
    }
}