use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde_json::to_string; // Import only to_string from serde_json
use serde_wasm_bindgen::from_value; // Import from serde_wasm_bindgen
use js_sys::Promise;
use rayon::prelude::*;

mod tests;
mod analise_senhas;

// Lista de senhas comuns
const COMMON_PASSWORDS: [&str; 5] = [
    "123456",
    "password",
    "123456789",
    "12345678",
    "qwerty",
];

// Função que avalia a força de uma senha
pub fn avaliar_forca(senha: &str) -> &str {
    if is_common_password(senha) {
        return "Muito Fraca";
    }

    if senha.len() < 8 {
        return "Fraca";
    }

    if senha.chars().any(|c| c.is_numeric())
        && senha.chars().any(|c| c.is_lowercase())
        && senha.chars().any(|c| c.is_uppercase())
        && senha.chars().any(|c| "!@#$%^&*()".contains(c))
    {
        return "Forte";
    }

    "Média"
}



// Função que verifica se a senha é comum
fn is_common_password(password: &str) -> bool {
    COMMON_PASSWORDS.contains(&password)
}


// Função interna que analisa as senhas usando Rayon para paralelização
pub fn analisar_senhas_internal(senhas: Vec<String>) -> Vec<String> {
    senhas
        .par_iter()
        .map(|senha| {
            let forca = avaliar_forca(senha);
            format!("Senha: {}, Força: {}", senha, forca)
        })
        .collect()
}
#[wasm_bindgen]
pub fn analisar_senhas(senhas_js: JsValue) -> Promise {
    let future = async move {
        let senhas: Vec<String> = match from_value(senhas_js) {
            Ok(senhas) => senhas,
            Err(err) => return Err(JsValue::from(err.to_string())),
        };

        let resultados = analisar_senhas_internal(senhas);
        let json_string = to_string(&resultados).unwrap();
        Ok(JsValue::from_str(&json_string))
    };

    future_to_promise(future)
}