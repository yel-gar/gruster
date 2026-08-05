const IGNORED_CHARS: &[char] = &['\n', ' '];

fn offset_string(s: String, offset: i32) -> String {
    s.as_bytes()
        .iter()
        .map(|&b| {
            if IGNORED_CHARS.contains(&(b as char)) {
                b as char
            } else {
                char::from_u32((b as i32 + offset) as u32).unwrap_or('?')
            }
        })
        .collect()
}

fn deobfuscate_from_wd(s: String) -> String {
    offset_string(s, -0xF000)
}

pub const ENTRY_6: &'static str = "";  // INTERESTING, HOW VERY INTERESTING
pub const ENTRY_26: &'static str = "";  // I SEEM TO SLOWLY GET
pub const ENTRY_28: &'static str = "";  // GREENER, YET GREENER

pub const ENTRY_67: &'static str = "";  // THIS IS VERY, VERY CRINGE
pub const ENTRY_191: &'static str = "";  // FREAKBOB IS COMING
