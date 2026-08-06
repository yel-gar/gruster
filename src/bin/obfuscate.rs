use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use rand::prelude::*;

const WD_OFFSET: i32 = 0xF000;
const EG_OFFSET: i32 = 0x13000;

fn main() {
    let txt = fs::read_to_string("entries.txt").unwrap();
    let mut output = Vec::with_capacity(txt.chars().filter(|&c| c == '\n').count());
    for line in txt.lines() {
        let (n, t) = line.split_once(": ").unwrap();
        let n_parsed = n.parse::<u64>().unwrap();
        let n_hashed = secret_hash(n_parsed);
        output.push(format!("({n_hashed}, \"{obfuscated}\"),  // {t}", obfuscated = obfuscate_to_wd(t.to_string())));
    }
    output.shuffle(&mut (rand::rng()));
    fs::write("entries_wd.txt", output.join("\n")).unwrap();
    // loop {
    //     print!("Enter string: ");
    //     std::io::stdout().flush().unwrap();
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     if input.is_empty() {
    //         break;
    //     }
    //     println!(
    //         "(0, \"{}\"),  // {input}",
    //         obfuscate_to_wd(input.clone())
    //     );
    // }
}

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

fn secret_hash(v: u64) -> u128 {
    simple_hash(v ^ 0x666 + 666)
}

fn offset_string(s: String, offset: i32) -> String {
    s.as_bytes()
        .iter()
        .map(|&b| char::from_u32((b as i32 + offset) as u32).unwrap_or('?'))
        .collect()
}

fn obfuscate_to_wd(s: String) -> String {
    offset_string(s, WD_OFFSET)
}

fn deobfuscate_from_wd(s: String) -> String {
    offset_string(s, -WD_OFFSET)
}
