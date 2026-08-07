use crate::secret::{FLAG_ARGON_NONCE, FLAG_ARGON_SALT, FLAG_PAYLOAD};
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use argon2::Argon2;
use egui::Color32;

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

pub fn color_from_lerp_f(lerp_f: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        (255.0 * (1.0 - lerp_f)) as u8,
        255,
        (255.0 * (1.0 - lerp_f)) as u8,
        255,
    )
}
