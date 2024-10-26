// src/utils.rs
use std::fs::File;
use std::io::{self, BufReader};
use serde_json::Value;
use csv::ReaderBuilder;

pub fn read_json(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let passwords: Value = serde_json::from_reader(reader)?;
    Ok(passwords.as_array().unwrap().iter().map(|p| p.as_str().unwrap().to_string()).collect())
}

pub fn read_csv(file_path: &str) -> Result<Vec<String>, io::Error> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut passwords = Vec::new();
    for result in rdr.records() {
        let record = result?;
        passwords.push(record[0].to_string());
    }
    Ok(passwords)
}
