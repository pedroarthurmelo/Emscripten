// Importação de bibliotecas necessárias
use wasm_bindgen::prelude::*; // Para interação com o JavaScript usando WebAssembly (WASM)
use wasm_bindgen_futures::future_to_promise; // Para converter o Future de Rust em Promise de JavaScript
use serde_json::to_string; // Importa apenas a função to_string da biblioteca serde_json para converter dados Rust em JSON
use serde_wasm_bindgen::from_value; // Para converter valores de JsValue em tipos Rust usando serde
use js_sys::Promise; // Para lidar com Promises em JavaScript
use rayon::prelude::*; // Importa Rayon para processamento paralelo

mod tests;
mod analise_senhas;

// Definição de uma lista constante de senhas comuns
const COMMON_PASSWORDS: [&str; 5] = [
    "123456",   // Exemplo de senha comum
    "password", // Senha fácil
    "123456789", // Senha comum
    "12345678",  // Outra senha fraca
    "qwerty",    // Senha com um padrão fácil de adivinhar
];

// Função que avalia a força de uma senha
// A função recebe uma senha e retorna um nível de força como uma string
pub fn avaliar_forca(senha: &str) -> &str {
    // Se a senha for uma das comuns, é considerada 'Muito Fraca'
    if is_common_password(senha) {
        return "Muito Fraca";
    }

    // Se a senha tem menos de 8 caracteres, é considerada 'Fraca'
    if senha.len() < 8 {
        return "Fraca";
    }

    // Se a senha contém números, letras minúsculas, maiúsculas e caracteres especiais,
    // é considerada 'Forte'
    if senha.chars().any(|c| c.is_numeric()) // Verifica se há números
        && senha.chars().any(|c| c.is_lowercase()) // Verifica se há letras minúsculas
        && senha.chars().any(|c| c.is_uppercase()) // Verifica se há letras maiúsculas
        && senha.chars().any(|c| "!@#$%^&*()".contains(c)) // Verifica se há caracteres especiais
    {
        return "Forte";
    }

    // Se não atende as condições anteriores, é considerada 'Média'
    "Média"
}

// Função auxiliar para verificar se uma senha é comum
// Retorna verdadeiro se a senha estiver na lista de senhas comuns
fn is_common_password(password: &str) -> bool {
    COMMON_PASSWORDS.contains(&password)
}

// Função que analisa uma lista de senhas usando Rayon para processamento paralelo
// Ela recebe um vetor de senhas e retorna um vetor de resultados formatados como strings
pub fn analisar_senhas_internal(senhas: Vec<String>) -> Vec<String> {
    // Usa a paralelização com Rayon para mapear cada senha na sua avaliação de força
    senhas
        .par_iter() // Itera sobre o vetor de senhas de forma paralela
        .map(|senha| { // Para cada senha, aplica a avaliação
            let forca = avaliar_forca(senha); // Avalia a força da senha
            format!("Senha: {}, Força: {}", senha, forca) // Formata o resultado
        })
        .collect() // Coleta os resultados em um vetor
}

// Função que será exposta para JavaScript usando WebAssembly
// Recebe um JsValue (que pode ser um vetor de senhas) e retorna uma Promise
#[wasm_bindgen]
pub fn analisar_senhas(senhas_js: JsValue) -> Promise {
    let future = async move { // Cria uma função assíncrona
        // Tenta converter o JsValue para um vetor de Strings em Rust
        let senhas: Vec<String> = match from_value(senhas_js) {
            Ok(senhas) => senhas, // Se a conversão for bem-sucedida, continua com as senhas
            Err(err) => return Err(JsValue::from(err.to_string())), // Caso contrário, retorna o erro
        };

        // Chama a função interna que analisa as senhas
        let resultados = analisar_senhas_internal(senhas);

        // Converte o vetor de resultados para uma string JSON
        let json_string = to_string(&resultados).unwrap(); // A conversão para JSON pode falhar
        Ok(JsValue::from_str(&json_string)) // Retorna o resultado como JsValue
    };

    // Converte o Future em uma Promise JavaScript e retorna
    future_to_promise(future)
}
