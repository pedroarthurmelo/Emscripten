// src/password_analysis.rs
use rayon::prelude::*;

pub fn analyze_passwords(passwords: Vec<String>) -> Vec<String> {
    passwords
        .par_iter()  // Usando multithreading
        .map(|password| {
            let strength = evaluate_strength(password);
            format!("Senha: {}, Força: {}", password, strength)
        })
        .collect()
}

fn evaluate_strength(password: &String) -> &str {
    if password.len() < 8 {
        return "Fraca";
    }
    if password.chars().any(|c| c.is_numeric()) &&
       password.chars().any(|c| c.is_lowercase()) &&
       password.chars().any(|c| c.is_uppercase()) &&
       password.chars().any(|c| "!@#$%^&*()".contains(c)) {
        return "Forte";
    }
    "Média"
}
