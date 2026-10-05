use argon2::Argon2;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    Key, XChaCha20Poly1305, XNonce,
};

pub fn derive_key(master_password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];

    Argon2::default()
        .hash_password_into(
            master_password.as_bytes(),
            salt,
            &mut key,
        )
        .expect("Failed to derive encryption key");

    key
}

pub fn encrypt_password(
    password: &str,
    key: &[u8; 32],
) -> String {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));

    let nonce = XNonce::generate();

    let ciphertext = cipher
        .encrypt(&nonce, password.as_bytes())
        .expect("Encryption failed");

    let mut encrypted_data = Vec::with_capacity(
        nonce.len() + ciphertext.len()
    );

    encrypted_data.extend_from_slice(&nonce);
    encrypted_data.extend_from_slice(&ciphertext);

    BASE64.encode(encrypted_data)
}

pub fn decrypt_password(
    encrypted_password: &str,
    key: &[u8; 32],
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_data = BASE64.decode(encrypted_password)?;

    if encrypted_data.len() < 24 {
        return Err("Encrypted data is too short".into());
    }

    let (nonce_bytes, ciphertext) =
        encrypted_data.split_at(24);

    let nonce = XNonce::from_slice(nonce_bytes);

    let cipher = XChaCha20Poly1305::new(
        Key::from_slice(key)
    );

    let plaintext = cipher.decrypt(
        nonce,
        ciphertext,
    )?;

    Ok(String::from_utf8(plaintext)?)
}