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
