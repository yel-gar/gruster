use aes_gcm::aead::{Aead, Generate};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use argon2::Argon2;

fn main() {
    encrypt();
    decrypt();
}

fn encrypt() {
    let ciphertext = b"letoctf{i_Am_v3R4_VERY_greeN_u3kHd9}";
    let password = b"ru5tgr3engas1er";
    let salt = b"im_also_a_pirate";
    let argon = Argon2::default();

    let mut key = [0u8; 32];
    argon.hash_password_into(password, salt, &mut key).unwrap();

    let nonce = Nonce::generate();
    let key = Key::<Aes256Gcm>::from_iter(key);
    let cipher = Aes256Gcm::new(&key);
    let encryped = cipher.encrypt(&nonce, ciphertext.as_ref()).unwrap();
    println!("nonce={nonce:?}; encryped={encryped:?}");
}

fn decrypt() {
    let nonce = [225u8, 146, 137, 51, 115, 72, 223, 59, 254, 5, 38, 70];
    let payload = [
        228u8, 47, 172, 90, 218, 207, 109, 6, 217, 113, 170, 39, 239, 77, 134, 71, 1, 140, 153, 67,
        77, 192, 33, 130, 19, 237, 154, 172, 2, 167, 131, 79, 221, 31, 46, 74, 113, 78, 41, 68, 49,
        2, 30, 166, 136, 204, 161, 185, 170, 44, 143, 37,
    ];
    let salt = b"im_also_a_pirate";
    let argon = Argon2::default();
    let password = b"ru5tgr3engas1er";
    let mut key = [0u8; 32];
    argon.hash_password_into(password, salt, &mut key).unwrap();

    let key = Key::<Aes256Gcm>::from_iter(key);
    let cipher = Aes256Gcm::new(&key);
    let decrypted = cipher
        .decrypt(&Nonce::from_iter(nonce), payload.as_ref())
        .unwrap();

    println!(
        "{}",
        decrypted.iter().map(|&c| c as char).collect::<String>()
    );
}
