use crate::secret::{FLAG_ARGON_NONCE, FLAG_ARGON_SALT, FLAG_PAYLOAD};
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use argon2::Argon2;

fn simple_hash(num: u64) -> u128 {
    const FNV_PRIME: u128 = 1000000000000066600000000000001;
    const FNV_OFFSET_BASIS: u128 = 14695981039346656037;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in num.to_le_bytes() {
        hash ^= byte as u128;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub fn secret_hash(v: u64) -> u128 {
    simple_hash(v ^ 0x666 + 666)
}

pub fn decrypt_flag(password: &String) -> String {
    let argon = Argon2::default();
    let mut key = [0u8; 32];
    argon
        .hash_password_into(password.as_bytes(), FLAG_ARGON_SALT, &mut key)
        .unwrap();

    let key = Key::<Aes256Gcm>::from_iter(key);
    let cipher = Aes256Gcm::new(&key);
    let decrypted = cipher
        .decrypt(&Nonce::from_iter(FLAG_ARGON_NONCE), FLAG_PAYLOAD.as_ref())
        .unwrap_or(b"letoctf{you_have_failed}".to_vec());

    decrypted.iter().map(|&b| b as char).collect()
}
