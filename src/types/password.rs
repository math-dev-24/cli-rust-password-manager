use serde::{Deserialize, Serialize};
use crate::utils::password::{encrypt_password, decrypt_password};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub id: String,
    password: Vec<u8>,
    pub description: String,
    nonce: [u8; 12],
}

impl PasswordEntry {
    pub fn new(id: String, password: String, description: String, key: &[u8; 32]) -> Self {
        let password = encrypt_password(&password, &key);
        PasswordEntry {
            id,
            password: password.0,
            description,
            nonce: password.1,
        }
    }
    pub fn get_password(&self, key: &[u8; 32]) -> String {
        decrypt_password(&self.password, &key, &self.nonce)
    }
}