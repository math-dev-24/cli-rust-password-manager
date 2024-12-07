use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::aead::rand_core::RngCore;


pub fn encrypt_password(password: &str, key: &[u8; 32]) -> (Vec<u8>, [u8; 12]) {
    let cipher = Aes256Gcm::new(key.into());
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), password.as_bytes())
        .expect("encryption failure!");
    (ciphertext, nonce)
}

pub fn decrypt_password(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> String {
    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher.decrypt(nonce.into(), ciphertext).expect("decryption failure!");
    String::from_utf8(plaintext).expect("Invalid UTF-8")
}