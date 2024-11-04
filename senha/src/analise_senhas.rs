// Importa a biblioteca Rayon para paralelização
use rayon::prelude::*;

// Lista de senhas comuns
const SENHAS_COMUNS: [&str; 5] = [
    "123456",
    "senha",
    "123456789",
    "12345678",
    "qwerty",
    // Adicione mais senhas comuns conforme necessário
];

// Função que recebe uma lista de senhas e analisa sua força usando multithreading
pub fn analisar_senhas(senhas: Vec<String>) -> Vec<String> {
    senhas
        .par_iter() // Usa iteração paralela para analisar as senhas em múltiplas threads
        .map(|senha| {
            let forca = avaliar_forca(senha); // Avalia a força de cada senha
            format!("Senha: {}, Força: {}", senha, forca) // Retorna uma string formatada com a senha e sua força
        })
        .collect() // Coleta os resultados em um vetor
}

// Função auxiliar que avalia a força de uma senha
pub fn avaliar_forca(senha: &String) -> &str {
    // Verifica se a senha é comum
    if is_senha_comum(senha) {
        return "Muito Fraca"; // Se a senha for comum, retorna "Muito Fraca"
    }
    
    // Senha fraca: menos de 8 caracteres
    if senha.len() < 8 {
        return "Fraca"; // Se a senha tem menos de 8 caracteres, retorna "Fraca"
    }
    
    // Senha forte: contém números, letras maiúsculas, minúsculas e caracteres especiais
    if senha.chars().any(|c| c.is_numeric()) && // Verifica se há números
       senha.chars().any(|c| c.is_lowercase()) && // Verifica se há letras minúsculas
       senha.chars().any(|c| c.is_uppercase()) && // Verifica se há letras maiúsculas
       senha.chars().any(|c| "!@#$%^&*()".contains(c)) { // Verifica se há caracteres especiais
        return "Forte"; // Se todas as condições forem atendidas, retorna "Forte"
    }
    
    // Caso contrário, considera como senha de força média
    "Média" // Se não se encaixar nas categorias anteriores, retorna "Média"
}

// Função que verifica se a senha é comum
fn is_senha_comum(senha: &String) -> bool {
    // Verifica se a senha está na lista de senhas comuns
    SENHAS_COMUNS.iter().any(|&comum| comum == senha.as_str())
}
