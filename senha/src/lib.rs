use wasm_bindgen::prelude::*; // Importa macros e utilitários necessários para trabalhar com WebAssembly
mod password_analysis; // Módulo que contém a lógica de análise de senhas
mod tests; // Módulo que contém os testes unitários

// Função exposta ao WebAssembly para ser chamada a partir de JavaScript
#[wasm_bindgen]
pub fn analyze_passwords(passwords: Vec<String>) -> Vec<String> {
    password_analysis::analyze_passwords(passwords) // Chama a função de análise de senhas do módulo password_analysis
}
