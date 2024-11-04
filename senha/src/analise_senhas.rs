// Importa a biblioteca Rayon para paralelização
use rayon::prelude::*; // Rayon é uma biblioteca para facilitar a programação paralela em Rust.
use std::collections::HashSet; // Importa o tipo HashSet, que é uma coleção que não permite elementos duplicados.

// Usando lazy_static para inicializar a lista de senhas comuns apenas uma vez, em tempo de execução.
lazy_static::lazy_static! {
    // Define uma variável estática chamada SENHAS_COMUNS que é um HashSet de strings estáticas.
    static ref SENHAS_COMUNS: HashSet<&'static str> = {
        let mut m = HashSet::new(); // Cria um novo HashSet vazio.
        // Adiciona senhas comuns ao HashSet.
        m.insert("123456");
        m.insert("senha");
        m.insert("123456789");
        m.insert("12345678");
        m.insert("qwerty");
        // Adicione mais senhas comuns conforme necessário.
        m // Retorna o HashSet preenchido.
    };
}

// Função que recebe uma lista de senhas e analisa sua força usando multithreading.
pub fn analisar_senhas(senhas: Vec<String>) -> Vec<String> {
    senhas
        .par_iter() // Usa iteração paralela para analisar as senhas em múltiplas threads.
        .map(|senha| {
            // Para cada senha, avalia sua força chamando a função avaliar_forca.
            let forca = avaliar_forca(senha.as_str()); // Converte String para &str para passar para a função.
            // Retorna uma string formatada com a senha e sua força.
            format!("Senha: {}, Força: {}", senha, forca)
        })
        .collect() // Coleta os resultados em um vetor de strings.
}

// Função auxiliar que avalia a força de uma senha.
pub fn avaliar_forca(senha: &str) -> &str {
    // Verifica se a senha é comum, usando a função is_senha_comum.
    if is_senha_comum(senha) {
        return "Muito Fraca"; // Se a senha for comum, retorna "Muito Fraca".
    }
    
    // Senha fraca: menos de 8 caracteres.
    if senha.len() < 8 {
        return "Fraca"; // Se a senha tem menos de 8 caracteres, retorna "Fraca".
    }
    
    // Inicializa as flags para verificar a força da senha.
    let mut tem_maiuscula = false; // Flag para indicar se há letras maiúsculas.
    let mut tem_minuscula = false; // Flag para indicar se há letras minúsculas.
    let mut tem_digito = false; // Flag para indicar se há dígitos.
    let mut tem_especial = false; // Flag para indicar se há caracteres especiais.

    // Itera pelos caracteres da senha.
    for c in senha.chars() {
        // Verifica se o caractere é uma letra maiúscula.
        if c.is_uppercase() {
            tem_maiuscula = true;
        // Verifica se o caractere é uma letra minúscula.
        } else if c.is_lowercase() {
            tem_minuscula = true;
        // Verifica se o caractere é um dígito.
        } else if c.is_numeric() {
            tem_digito = true;
        // Verifica se o caractere é um dos caracteres especiais definidos.
        } else if "!@#$%^&*()".contains(c) {
            tem_especial = true;
        }

        // Se todas as condições foram atendidas, não precisamos continuar a iteração.
        if tem_maiuscula && tem_minuscula && tem_digito && tem_especial {
            return "Forte"; // Se todas as condições forem atendidas, retorna "Forte".
        }
    }

    // Caso contrário, considera como senha de força média.
    "Média" // Se não se encaixar nas categorias anteriores, retorna "Média".
}

// Função que verifica se a senha é comum.
fn is_senha_comum(senha: &str) -> bool {
    // Verifica se a senha está na lista de senhas comuns.
    SENHAS_COMUNS.contains(senha) // Usando HashSet para busca eficiente.
}
