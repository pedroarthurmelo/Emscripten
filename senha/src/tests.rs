#[cfg(test)]
mod tests {
    use crate::password_analysis::evaluate_strength; // Importa a função de avaliação de força de senhas para os testes

    // Testa uma senha fraca (menos de 8 caracteres)
    #[test]
    fn test_weak_password() {
        assert_eq!(evaluate_strength(&String::from("123456")), "Fraca"); // A senha "123456" deve ser considerada fraca
    }

    // Testa uma senha de força média (contém números e letras, mas sem caracteres especiais)
    #[test]
    fn test_medium_password() {
        assert_eq!(evaluate_strength(&String::from("Password1")), "Média"); // A senha "Password1" deve ser considerada de força média
    }

    // Testa uma senha forte (contém números, letras e caracteres especiais)
    #[test]
    fn test_strong_password() {
        assert_eq!(evaluate_strength(&String::from("P@ssw0rd!")), "Forte"); // A senha "P@ssw0rd!" deve ser considerada forte
    }
}
