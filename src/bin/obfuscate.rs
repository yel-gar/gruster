use std::io::Write;

const WD_OFFSET: i32 = 0xF000;
const EG_OFFSET: i32 = 0x13000;

fn main() {
    loop {
        print!("Enter string: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        }
        println!(
            "pub const ENTRY_x: &'static str = \"{}\";  // {input}",
            obfuscate_to_wd(input.clone())
        );
    }
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
