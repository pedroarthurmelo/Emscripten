use rayon::prelude::*; // Importa a biblioteca Rayon para paralelização

// Lista de senhas comuns
const COMMON_PASSWORDS: [&str; 5] = [
    "123456",
    "password",
    "123456789",
    "12345678",
    "qwerty",
    // Adicione mais senhas comuns conforme necessário
];

// Função que recebe uma lista de senhas e analisa sua força usando multithreading
pub fn analyze_passwords(passwords: Vec<String>) -> Vec<String> {
    passwords
        .par_iter() // Usa iteração paralela para analisar as senhas em múltiplas threads
        .map(|password| {
            let strength = evaluate_strength(password); // Avalia a força de cada senha
            format!("Senha: {}, Força: {}", password, strength) // Retorna uma string formatada com a senha e sua força
        })
        .collect() // Coleta os resultados em um vetor
}

// Função auxiliar que avalia a força de uma senha
pub fn evaluate_strength(password: &String) -> &str {
    // Verifica se a senha é comum
    if is_common_password(password) {
        return "Muito Fraca";
    }
    
    // Senha fraca: menos de 8 caracteres
    if password.len() < 8 {
        return "Fraca";
    }
    
    // Senha forte: contém números, letras maiúsculas, minúsculas e caracteres especiais
    if password.chars().any(|c| c.is_numeric()) && // Verifica se há números
       password.chars().any(|c| c.is_lowercase()) && // Verifica se há letras minúsculas
       password.chars().any(|c| c.is_uppercase()) && // Verifica se há letras maiúsculas
       password.chars().any(|c| "!@#$%^&*()".contains(c)) { // Verifica se há caracteres especiais
        return "Forte";
    }
    
    // Caso contrário, considera como senha de força média
    "Média"
}

// Função que verifica se a senha é comum
fn is_common_password(password: &String) -> bool {
    COMMON_PASSWORDS.iter().any(|&common| common == password.as_str())
}
