// src/tests.rs
#[cfg(test)]
mod tests {
    use super::password_analysis::evaluate_strength;

    #[test]
    fn test_weak_password() {
        assert_eq!(evaluate_strength(&String::from("123456")), "Fraca");
    }

    #[test]
    fn test_medium_password() {
        assert_eq!(evaluate_strength(&String::from("Password1")), "Média");
    }

    #[test]
    fn test_strong_password() {
        assert_eq!(evaluate_strength(&String::from("P@ssw0rd!")), "Forte");
    }
}
