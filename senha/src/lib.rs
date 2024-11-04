// Importa macros e utilitários necessários para trabalhar com WebAssembly
use wasm_bindgen::prelude::*;

// Módulo que contém a lógica de análise de senhas
mod analise_senhas;

//modulo para testes
mod tests;

// Função exposta ao WebAssembly para ser chamada a partir de JavaScript
#[wasm_bindgen]
pub fn analisar_senhas(senhas: Vec<String>) -> Vec<String> {
    // Chama a função de análise de senhas do módulo analise_senhas
    analise_senhas::analisar_senhas(senhas)
}
