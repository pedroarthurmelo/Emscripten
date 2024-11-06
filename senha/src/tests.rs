use crate::avaliar_forca;
use crate::analisar_senhas_internal as analyze_passwords;
// Módulo de testes
#[cfg(test)]
mod testes {
    // Importa a função de avaliação de força de senhas para os testes

    use crate::avaliar_forca;

// Testa uma senha fraca (menos de 8 caracteres)
#[test]
fn teste_senha_fraca() {
    // Verifica se a senha "abcde" (fraca) é considerada "Fraca"
    assert_eq!(avaliar_forca(&String::from("abcde")), "Fraca");
}

// Testa uma senha de força média (contém números e letras, mas sem caracteres especiais)
#[test]
fn teste_senha_media() {
    // Verifica se a senha "Password1" (válida para média) é considerada "Média"
    assert_eq!(avaliar_forca(&String::from("Password1")), "Média");
}

// Testa uma senha forte (contém números, letras e caracteres especiais)
#[test]
fn teste_senha_forte() {
    // Verifica se a senha "P@ssw0rd!" é considerada "Forte"
    assert_eq!(avaliar_forca(&String::from("P@ssw0rd!")), "Forte");
    // Verifica se a senha "Str0ng@Password" é considerada "Forte"
    assert_eq!(avaliar_forca(&String::from("Str0ng@Password")), "Forte");
    }
}
