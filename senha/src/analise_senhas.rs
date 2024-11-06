use rayon::prelude::*; // Importa a biblioteca Rayon para paralelização, permitindo que as operações em iteradores sejam feitas em múltiplas threads.

/// Lista de senhas comuns
/// A lista contém algumas senhas amplamente conhecidas e fáceis de adivinhar.
/// Pode ser expandida conforme necessário.
const COMMON_PASSWORDS: [&str; 5] = [
    "123456",   // Senha muito comum
    "password", // Outra senha fraca e comum
    "123456789", // Senha comum
    "12345678",  // Mais uma senha comum
    "qwerty",    // Padrão de teclado comum
    // Adicione mais senhas comuns conforme necessário
];

/// Função que recebe uma lista de senhas e analisa sua força usando multithreading (paralelização).
/// Ela recebe um vetor de senhas e retorna um vetor de strings com as senhas e suas respectivas forças.
pub fn analyze_passwords(passwords: Vec<String>) -> Vec<String> {
    passwords
        .par_iter() // Usa iteração paralela para processar as senhas em múltiplas threads.
        .map(|password| {
            let strength = evaluate_strength(password); // Avalia a força de cada senha.
            format!("Senha: {}, Força: {}", password, strength) // Retorna uma string formatada com a senha e a força.
        })
        .collect() // Coleta os resultados em um vetor de strings.
}

/// Função auxiliar que avalia a força de uma senha.
/// Retorna uma classificação de força: "Muito Fraca", "Fraca", "Média" ou "Forte".
pub fn evaluate_strength(password: &str) -> &str {
    // Verifica se a senha é uma das comuns
    if is_common_password(password) {
        return "Muito Fraca"; // Se for uma senha comum, classifica como "Muito Fraca".
    }
    
    // Senha fraca: menos de 8 caracteres
    if password.len() < 8 {
        return "Fraca"; // Se a senha tiver menos de 8 caracteres, classifica como "Fraca".
    }
    
    // Senha forte: contém números, letras minúsculas, maiúsculas e caracteres especiais
    if password.chars().any(|c| c.is_numeric()) && // Verifica se há números na senha.
       password.chars().any(|c| c.is_lowercase()) && // Verifica se há letras minúsculas.
       password.chars().any(|c| c.is_uppercase()) && // Verifica se há letras maiúsculas.
       password.chars().any(|c| "!@#$%^&*()".contains(c)) { // Verifica se há caracteres especiais na senha.
        return "Forte"; // Se todas as condições forem atendidas, a senha é considerada "Forte".
    }
    
    // Caso contrário, considera como senha de força média
    "Média" // Se a senha não atender às condições de "Muito Fraca", "Fraca" ou "Forte", é classificada como "Média".
}

/// Função que verifica se a senha está na lista de senhas comuns.
/// Retorna verdadeiro se a senha for uma das senhas comuns definidas na constante.
fn is_common_password(password: &str) -> bool {
    COMMON_PASSWORDS.contains(&password) // Verifica se a senha está na lista de senhas comuns.
}
