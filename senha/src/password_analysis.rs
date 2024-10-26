use rayon::prelude::*; // Importa a biblioteca Rayon para paralelização

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
