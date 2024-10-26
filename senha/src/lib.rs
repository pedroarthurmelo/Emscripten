// src/lib.rs
use wasm_bindgen::prelude::*;
mod password_analysis;

#[wasm_bindgen]
pub fn analyze_passwords(passwords: Vec<String>) -> Vec<String> {
    password_analysis::analyze_passwords(passwords)
}
