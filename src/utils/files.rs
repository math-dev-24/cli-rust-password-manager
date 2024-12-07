use std::fs;
use std::io::Write;
use crate::PasswordEntry;

// Load passwords from file
pub fn load_password(file_path: &str) -> Result<Vec<PasswordEntry>, std::io::Error> {
    if !std::path::Path::new(file_path).exists() {
        let mut file = fs::File::create(file_path)?;
        file.write_all(b"[]")?;
    }
    let json = fs::read_to_string(file_path)?;
    let passwords: Vec<PasswordEntry> = serde_json::from_str(&json)?;
    Ok(passwords)
}

// Save passwords to a file
pub fn save_password(passwords: &[PasswordEntry], file_path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(passwords)?;
    fs::write(file_path, json)?;
    Ok(())
}