use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

use crate::error::AppError;

const KEY_FILE: &str = "encryption.key";

pub struct EncryptionKey {
    pub key: [u8; 32],
}

impl EncryptionKey {
    pub fn load_or_create(app_handle: &AppHandle) -> Self {
        let key_path = Self::key_path(app_handle);

        if key_path.exists() {
            let encoded = fs::read_to_string(&key_path).expect("Failed to read encryption key");
            let bytes = BASE64
                .decode(encoded.trim())
                .expect("Failed to decode encryption key");
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            Self { key }
        } else {
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);
            let encoded = BASE64.encode(key);
            if let Some(parent) = key_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create key directory");
            }
            fs::write(&key_path, encoded).expect("Failed to write encryption key");
            Self { key }
        }
    }

    fn key_path(app_handle: &AppHandle) -> PathBuf {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to resolve app data dir");
        app_dir.join(KEY_FILE)
    }
}

pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>), AppError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| AppError::Crypto(e.to_string()))?;

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| AppError::Crypto(e.to_string()))?;

    Ok((ciphertext, nonce_bytes.to_vec()))
}

pub fn decrypt(key: &[u8; 32], ciphertext: &[u8], nonce_bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| AppError::Crypto(e.to_string()))?;

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| AppError::Crypto(e.to_string()))
}

pub fn mask_api_key(raw_key: &str) -> String {
    if raw_key.len() <= 8 {
        return "*".repeat(raw_key.len());
    }
    let prefix = &raw_key[..4];
    let suffix = &raw_key[raw_key.len() - 4..];
    format!("{}****{}", prefix, suffix)
}
